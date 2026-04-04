<script lang="ts">
	import { T, useTask, useThrelte } from "@threlte/core";
	import { useTexture } from "@threlte/extras";
	import * as THREE from "three";

	interface Props {
		/**
		 * Source URL of the base image.
		 */
		baseImage: string;
		/**
		 * Source URL of the image revealed by the fluid mask.
		 */
		revealImage: string;
		/**
		 * Dissipation factor for the reveal mask.
		 * @default 0.96
		 */
		dissipation?: number;
		/**
		 * Radius of the pointer influence.
		 * @default 0.005
		 */
		pointerSize?: number;
		/**
		 * Fluid velocity dissipation.
		 * @default 0.96
		 */
		velocityDissipation?: number;
		/**
		 * Pressure iterations. More iterations = more accurate but slower.
		 * @default 10
		 */
		pressureIterations?: number;
		/**
		 * Softness of the reveal transition edge.
		 * @default 0.22
		 */
		blendSoftness?: number;
	}

	type PointerState = {
		x: number;
		y: number;
		dx: number;
		dy: number;
		moved: boolean;
		initialized: boolean;
	};

	type CanvasMetrics = {
		width: number;
		height: number;
	};

	let {
		baseImage,
		revealImage,
		dissipation = 0.96,
		pointerSize = 0.005,
		velocityDissipation = 0.96,
		pressureIterations = 10,
		blendSoftness = 0.22,
	}: Props = $props();

	const { renderer, size } = useThrelte();
	const pointerState = $state<PointerState>({
		x: 0,
		y: 0,
		dx: 0,
		dy: 0,
		moved: false,
		initialized: false,
	});
	const canvasMetrics = $state<CanvasMetrics>({
		width: 1,
		height: 1,
	});

	const pointerUv = new THREE.Vector2();
	const baseTextureSize = new THREE.Vector2(1, 1);
	const revealTextureSize = new THREE.Vector2(1, 1);

	const pointerForceClamp = 450;
	const pointerForceInitialLerp = 0.2;
	const pointerForceLerp = 0.55;

	const baseTexture = $derived(useTexture(baseImage));
	const revealTexture = $derived(useTexture(revealImage));

	$effect(() => {
		const tex = $baseTexture;
		if (tex?.image) {
			baseTextureSize.set(tex.image.width || 1, tex.image.height || 1);
		}
	});

	$effect(() => {
		const tex = $revealTexture;
		if (tex?.image) {
			revealTextureSize.set(tex.image.width || 1, tex.image.height || 1);
		}
	});

	const updatePointerPosition = (
		px: number,
		py: number,
		width: number,
		height: number,
	) => {
		const prevX = pointerState.x;
		const prevY = pointerState.y;
		const targetDx = THREE.MathUtils.clamp(
			5 * (px - prevX),
			-pointerForceClamp,
			pointerForceClamp,
		);
		const targetDy = THREE.MathUtils.clamp(
			5 * (py - prevY),
			-pointerForceClamp,
			pointerForceClamp,
		);
		const lerpFactor = pointerState.initialized
			? pointerForceLerp
			: pointerForceInitialLerp;

		pointerState.moved = true;
		pointerState.dx = THREE.MathUtils.lerp(
			pointerState.dx,
			targetDx,
			lerpFactor,
		);
		pointerState.dy = THREE.MathUtils.lerp(
			pointerState.dy,
			targetDy,
			lerpFactor,
		);
		pointerState.x = px;
		pointerState.y = py;
		pointerState.initialized = true;

		if (width > 0 && height > 0) {
			pointerUv.set(px / width, 1 - py / height);
		}
	};

	const vertexShader = `
		varying vec2 vUv;
		varying vec2 vL;
		varying vec2 vR;
		varying vec2 vT;
		varying vec2 vB;
		uniform vec2 uTexel;

		void main () {
			vUv = uv;
			vL = vUv - vec2(uTexel.x, 0.);
			vR = vUv + vec2(uTexel.x, 0.);
			vT = vUv + vec2(0., uTexel.y);
			vB = vUv - vec2(0., uTexel.y);
			gl_Position = vec4(position, 1.0);
		}
	`;

	const advectionShader = `
		varying vec2 vUv;
		uniform sampler2D uVelocity;
		uniform sampler2D uInput;
		uniform vec2 uTexel;
		uniform float uDt;
		uniform float uDissipation;

		vec4 bilerp (sampler2D sam, vec2 uv, vec2 tsize) {
			vec2 st = uv / tsize - 0.5;
			vec2 iuv = floor(st);
			vec2 fuv = fract(st);
			vec4 a = texture2D(sam, (iuv + vec2(0.5, 0.5)) * tsize);
			vec4 b = texture2D(sam, (iuv + vec2(1.5, 0.5)) * tsize);
			vec4 c = texture2D(sam, (iuv + vec2(0.5, 1.5)) * tsize);
			vec4 d = texture2D(sam, (iuv + vec2(1.5, 1.5)) * tsize);
			return mix(mix(a, b, fuv.x), mix(c, d, fuv.x), fuv.y);
		}

		void main () {
			vec2 coord = vUv - uDt * bilerp(uVelocity, vUv, uTexel).xy * uTexel;
			gl_FragColor = uDissipation * bilerp(uInput, coord, uTexel);
			gl_FragColor.a = 1.;
		}
	`;

	const divergenceShader = `
		varying highp vec2 vUv;
		varying highp vec2 vL;
		varying highp vec2 vR;
		varying highp vec2 vT;
		varying highp vec2 vB;
		uniform sampler2D uVelocity;

		void main () {
			float L = texture2D(uVelocity, vL).x;
			float R = texture2D(uVelocity, vR).x;
			float T = texture2D(uVelocity, vT).y;
			float B = texture2D(uVelocity, vB).y;
			float div = .6 * (R - L + T - B);
			gl_FragColor = vec4(div, 0., 0., 1.);
		}
	`;

	const pressureShader = `
		varying highp vec2 vUv;
		varying highp vec2 vL;
		varying highp vec2 vR;
		varying highp vec2 vT;
		varying highp vec2 vB;
		uniform sampler2D uPressure;
		uniform sampler2D uDivergence;

		void main () {
			float L = texture2D(uPressure, vL).x;
			float R = texture2D(uPressure, vR).x;
			float T = texture2D(uPressure, vT).x;
			float B = texture2D(uPressure, vB).x;
			float divergence = texture2D(uDivergence, vUv).x;
			float pressure = (L + R + B + T - divergence) * 0.25;
			gl_FragColor = vec4(pressure, 0., 0., 1.);
		}
	`;

	const gradientSubtractShader = `
		varying highp vec2 vUv;
		varying highp vec2 vL;
		varying highp vec2 vR;
		varying highp vec2 vT;
		varying highp vec2 vB;
		uniform sampler2D uPressure;
		uniform sampler2D uVelocity;

		void main () {
			float L = texture2D(uPressure, vL).x;
			float R = texture2D(uPressure, vR).x;
			float T = texture2D(uPressure, vT).x;
			float B = texture2D(uPressure, vB).x;
			vec2 velocity = texture2D(uVelocity, vUv).xy;
			velocity.xy -= vec2(R - L, T - B);
			gl_FragColor = vec4(velocity, 0., 1.);
		}
	`;

	const splatShader = `
		varying vec2 vUv;
		uniform sampler2D uInput;
		uniform float uRatio;
		uniform vec3 uPointValue;
		uniform vec2 uPoint;
		uniform float uPointSize;

		void main () {
			vec2 p = vUv - uPoint.xy;
			p.x *= uRatio;
			vec3 splat = pow(2., -dot(p, p) / uPointSize) * uPointValue;
			vec3 base = texture2D(uInput, vUv).xyz;
			gl_FragColor = vec4(base + splat, 1.);
		}
	`;

	const outputShader = `
		varying vec2 vUv;
		uniform sampler2D uMaskTexture;
		uniform sampler2D uBaseTexture;
		uniform sampler2D uRevealTexture;
		uniform vec2 uResolution;
		uniform vec2 uBaseTextureSize;
		uniform vec2 uRevealTextureSize;
		uniform vec2 uMaskTexel;
		uniform float uBlendSoftness;

		vec2 getCoverUV(vec2 uv, vec2 textureSize) {
			vec2 s = uResolution / textureSize;
			float scale = max(s.x, s.y);
			vec2 scaledSize = textureSize * scale;
			vec2 offset = (uResolution - scaledSize) * 0.5;
			return (uv * uResolution - offset) / scaledSize;
		}

		float sampleMask(vec2 uv) {
			vec3 maskData = texture2D(uMaskTexture, uv).rgb;
			return clamp(max(maskData.r, max(maskData.g, maskData.b)), 0.0, 1.0);
		}

		float getSmoothMask(vec2 uv) {
			vec2 t = uMaskTexel;
			float m = 0.0;
			m += sampleMask(uv + vec2(-t.x, -t.y)) * 1.0;
			m += sampleMask(uv + vec2(0.0, -t.y)) * 2.0;
			m += sampleMask(uv + vec2(t.x, -t.y)) * 1.0;
			m += sampleMask(uv + vec2(-t.x, 0.0)) * 2.0;
			m += sampleMask(uv) * 4.0;
			m += sampleMask(uv + vec2(t.x, 0.0)) * 2.0;
			m += sampleMask(uv + vec2(-t.x, t.y)) * 1.0;
			m += sampleMask(uv + vec2(0.0, t.y)) * 2.0;
			m += sampleMask(uv + vec2(t.x, t.y)) * 1.0;
			return m / 16.0;
		}

		void main () {
			vec2 baseUv = getCoverUV(vUv, uBaseTextureSize);
			vec2 revealUv = getCoverUV(vUv, uRevealTextureSize);

			vec3 baseColor = texture2D(uBaseTexture, baseUv).rgb;
			vec3 revealColor = texture2D(uRevealTexture, revealUv).rgb;

			float rawMask = getSmoothMask(vUv);
			float softness = clamp(uBlendSoftness, 0.01, 0.49);
			float mask = smoothstep(0.5 - softness, 0.5 + softness, rawMask);

			vec3 color = mix(baseColor, revealColor, mask);
			gl_FragColor = vec4(color, 1.0);
			#include <colorspace_fragment>
		}
	`;

	const createFBO = (w: number, h: number) =>
		new THREE.WebGLRenderTarget(w, h, {
			type: THREE.FloatType,
			minFilter: THREE.NearestFilter,
			magFilter: THREE.NearestFilter,
			format: THREE.RGBAFormat,
		});

	let density = $state({
		read: createFBO(128, 128),
		write: createFBO(128, 128),
		swap: () => {
			const temp = density.read;
			density.read = density.write;
			density.write = temp;
		},
	});

	let velocity = $state({
		read: createFBO(128, 128),
		write: createFBO(128, 128),
		swap: () => {
			const temp = velocity.read;
			velocity.read = velocity.write;
			velocity.write = temp;
		},
	});

	let pressure = $state({
		read: createFBO(128, 128),
		write: createFBO(128, 128),
		swap: () => {
			const temp = pressure.read;
			pressure.read = pressure.write;
			pressure.write = temp;
		},
	});

	let divergence = $state(createFBO(128, 128));

	const advectionMat = new THREE.ShaderMaterial({
		uniforms: {
			uVelocity: { value: null },
			uInput: { value: null },
			uTexel: { value: new THREE.Vector2() },
			uDt: { value: 0.016 },
			uDissipation: { value: 0.96 },
		},
		vertexShader,
		fragmentShader: advectionShader,
	});

	const divergenceMat = new THREE.ShaderMaterial({
		uniforms: {
			uVelocity: { value: null },
			uTexel: { value: new THREE.Vector2() },
		},
		vertexShader,
		fragmentShader: divergenceShader,
	});

	const pressureMat = new THREE.ShaderMaterial({
		uniforms: {
			uPressure: { value: null },
			uDivergence: { value: null },
			uTexel: { value: new THREE.Vector2() },
		},
		vertexShader,
		fragmentShader: pressureShader,
	});

	const gradientSubtractMat = new THREE.ShaderMaterial({
		uniforms: {
			uPressure: { value: null },
			uVelocity: { value: null },
			uTexel: { value: new THREE.Vector2() },
		},
		vertexShader,
		fragmentShader: gradientSubtractShader,
	});

	const splatMat = new THREE.ShaderMaterial({
		uniforms: {
			uInput: { value: null },
			uRatio: { value: 1 },
			uPointValue: { value: new THREE.Vector3() },
			uPoint: { value: new THREE.Vector2() },
			uPointSize: { value: 0.01 },
		},
		vertexShader,
		fragmentShader: splatShader,
	});

	let outputMat: THREE.ShaderMaterial | undefined = $state();

	const simScene = new THREE.Scene();
	const simCamera = new THREE.OrthographicCamera(-1, 1, 1, -1, 0, 1);
	const simMesh = new THREE.Mesh(new THREE.PlaneGeometry(2, 2), advectionMat);
	simScene.add(simMesh);

	const handlePointerMove = (e: PointerEvent) => {
		const rect = renderer.domElement.getBoundingClientRect();
		const x = e.clientX - rect.left;
		const y = e.clientY - rect.top;
		updatePointerPosition(x, y, rect.width, rect.height);
	};

	const handleTouchMove = (e: TouchEvent) => {
		e.preventDefault();
		const touch = e.touches[0];
		if (!touch) return;
		const rect = renderer.domElement.getBoundingClientRect();
		const x = touch.clientX - rect.left;
		const y = touch.clientY - rect.top;
		updatePointerPosition(x, y, rect.width, rect.height);
	};

	$effect(() => {
		const canvas = renderer.domElement;
		canvas.addEventListener("pointermove", handlePointerMove);
		canvas.addEventListener("touchmove", handleTouchMove, { passive: false });
		return () => {
			canvas.removeEventListener("pointermove", handlePointerMove);
			canvas.removeEventListener("touchmove", handleTouchMove);
		};
	});

	$effect(() => {
		const base = $baseTexture;
		const reveal = $revealTexture;
		if (!outputMat) return;
		outputMat.uniforms.uBaseTexture.value = base ?? null;
		outputMat.uniforms.uRevealTexture.value = reveal ?? null;
	});

	$effect(() => {
		return () => {
			density.read.dispose();
			density.write.dispose();
			velocity.read.dispose();
			velocity.write.dispose();
			pressure.read.dispose();
			pressure.write.dispose();
			divergence.dispose();

			advectionMat.dispose();
			divergenceMat.dispose();
			pressureMat.dispose();
			gradientSubtractMat.dispose();
			splatMat.dispose();
			simMesh.geometry.dispose();
		};
	});

	$effect(() => {
		const w = Math.max(1, $size.width || renderer.domElement.clientWidth || 1);
		const h = Math.max(
			1,
			$size.height || renderer.domElement.clientHeight || 1,
		);
		canvasMetrics.width = w;
		canvasMetrics.height = h;

		const simResX = Math.max(1, Math.floor(w * 0.5));
		const simResY = Math.max(1, Math.floor(h * 0.5));

		const resizeFBO = (fbo: THREE.WebGLRenderTarget) => {
			fbo.setSize(simResX, simResY);
		};

		resizeFBO(density.read);
		resizeFBO(density.write);
		resizeFBO(velocity.read);
		resizeFBO(velocity.write);
		resizeFBO(pressure.read);
		resizeFBO(pressure.write);
		resizeFBO(divergence);

		const texelX = 1.0 / simResX;
		const texelY = 1.0 / simResY;
		const texel = new THREE.Vector2(texelX, texelY);

		advectionMat.uniforms.uTexel.value = texel;
		divergenceMat.uniforms.uTexel.value = texel;
		pressureMat.uniforms.uTexel.value = texel;
		gradientSubtractMat.uniforms.uTexel.value = texel;

		if (outputMat) {
			outputMat.uniforms.uResolution.value.set(w, h);
			outputMat.uniforms.uBaseTextureSize.value.copy(baseTextureSize);
			outputMat.uniforms.uRevealTextureSize.value.copy(revealTextureSize);
			outputMat.uniforms.uMaskTexel.value.set(texelX, texelY);
		}

		if (canvasMetrics.width > 0 && canvasMetrics.height > 0) {
			pointerUv.set(
				pointerState.x / canvasMetrics.width,
				1 - pointerState.y / canvasMetrics.height,
			);
		}
	});

	useTask(() => {
		const dt = 1 / 60;
		const width =
			canvasMetrics.width ||
			renderer.domElement.clientWidth ||
			renderer.domElement.width ||
			1;
		const height =
			canvasMetrics.height ||
			renderer.domElement.clientHeight ||
			renderer.domElement.height ||
			1;
		const aspect = height > 0 ? width / height : 1;

		if (pointerState.moved) {
			simMesh.material = splatMat;
			splatMat.uniforms.uInput.value = velocity.read.texture;
			splatMat.uniforms.uRatio.value = aspect;
			splatMat.uniforms.uPoint.value = pointerUv;
			splatMat.uniforms.uPointValue.value.set(
				pointerState.dx,
				-pointerState.dy,
				1,
			);
			splatMat.uniforms.uPointSize.value = pointerSize ?? 0.005;

			renderer.setRenderTarget(velocity.write);
			renderer.render(simScene, simCamera);
			velocity.swap();

			simMesh.material = splatMat;
			splatMat.uniforms.uInput.value = density.read.texture;
			splatMat.uniforms.uPointValue.value.set(1, 1, 1);

			renderer.setRenderTarget(density.write);
			renderer.render(simScene, simCamera);
			density.swap();
			pointerState.moved = false;
		}

		simMesh.material = divergenceMat;
		divergenceMat.uniforms.uVelocity.value = velocity.read.texture;
		renderer.setRenderTarget(divergence);
		renderer.render(simScene, simCamera);

		simMesh.material = pressureMat;
		pressureMat.uniforms.uDivergence.value = divergence.texture;
		for (let i = 0; i < pressureIterations; i++) {
			pressureMat.uniforms.uPressure.value = pressure.read.texture;
			renderer.setRenderTarget(pressure.write);
			renderer.render(simScene, simCamera);
			pressure.swap();
		}

		simMesh.material = gradientSubtractMat;
		gradientSubtractMat.uniforms.uPressure.value = pressure.read.texture;
		gradientSubtractMat.uniforms.uVelocity.value = velocity.read.texture;
		renderer.setRenderTarget(velocity.write);
		renderer.render(simScene, simCamera);
		velocity.swap();

		simMesh.material = advectionMat;
		advectionMat.uniforms.uVelocity.value = velocity.read.texture;
		advectionMat.uniforms.uInput.value = velocity.read.texture;
		advectionMat.uniforms.uDissipation.value = velocityDissipation;
		advectionMat.uniforms.uDt.value = dt;
		renderer.setRenderTarget(velocity.write);
		renderer.render(simScene, simCamera);
		velocity.swap();

		simMesh.material = advectionMat;
		advectionMat.uniforms.uVelocity.value = velocity.read.texture;
		advectionMat.uniforms.uInput.value = density.read.texture;
		advectionMat.uniforms.uDissipation.value = dissipation;
		renderer.setRenderTarget(density.write);
		renderer.render(simScene, simCamera);
		density.swap();

		renderer.setRenderTarget(null);

		if (outputMat) {
			outputMat.uniforms.uMaskTexture.value = density.read.texture;
			outputMat.uniforms.uResolution.value.set(width, height);
			outputMat.uniforms.uBaseTextureSize.value.copy(baseTextureSize);
			outputMat.uniforms.uRevealTextureSize.value.copy(revealTextureSize);
			outputMat.uniforms.uBlendSoftness.value = blendSoftness;
		}
	});
</script>

{#if $baseTexture && $revealTexture}
	<T.Mesh>
		<T.PlaneGeometry args={[2, 2]} />
		<T.ShaderMaterial
			bind:ref={outputMat}
			{vertexShader}
			fragmentShader={outputShader}
			uniforms={{
				uMaskTexture: { value: null },
				uBaseTexture: { value: $baseTexture },
				uRevealTexture: { value: $revealTexture },
				uResolution: { value: new THREE.Vector2(1, 1) },
				uBaseTextureSize: { value: baseTextureSize },
				uRevealTextureSize: { value: revealTextureSize },
				uMaskTexel: { value: new THREE.Vector2(1, 1) },
				uBlendSoftness: { value: blendSoftness },
			}}
			side={THREE.DoubleSide}
		/>
	</T.Mesh>
{/if}
