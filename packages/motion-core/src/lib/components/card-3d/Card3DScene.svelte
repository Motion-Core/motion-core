<script lang="ts">
	import { onMount } from "svelte";
	import {
		Box,
		Camera,
		Mesh,
		Program,
		Renderer,
		Texture,
		Transform,
	} from "ogl";

	interface HeadPosition {
		x: number;
		y: number;
		z: number;
	}

	interface Props {
		/**
		 * The image source URL.
		 */
		image: string;
		/**
		 * Width of the card.
		 * @default 3.2
		 */
		width?: number;
		/**
		 * Height of the card.
		 * @default 2
		 */
		height?: number;
		/**
		 * Depth/thickness of the card.
		 * @default 0.08
		 */
		depth?: number;
		/**
		 * Corner radius of the card.
		 * @default 0.15
		 */
		radius?: number;
		/**
		 * Head position for parallax effect.
		 */
		headPosition?: HeadPosition;
	}

	let {
		image,
		width = 3.2,
		height = 2,
		depth = 0.08,
		radius = 0.15,
		headPosition = { x: 0, y: 0, z: 0 },
	}: Props = $props();

	let canvas = $state<HTMLCanvasElement>();
	let setDimensions =
		$state<
			(next: {
				width: number;
				height: number;
				depth: number;
				radius: number;
			}) => void
		>();
	let setImageSource = $state<(source: string) => void>();

	const initialCameraPosition = { x: 0, y: 0, z: 5 };
	const lerpFactor = 0.1;
	let smoothedRotation = { x: 0, y: 0 };

	const createRoundedCardGeometry = (
		gl: Renderer["gl"],
		cardWidth: number,
		cardHeight: number,
		cardDepth: number,
		cardRadius: number,
	) => {
		const widthSegments = Math.max(12, Math.round(cardWidth * 10));
		const heightSegments = Math.max(12, Math.round(cardHeight * 10));
		const depthSegments = 2;

		const geometry = new Box(gl, {
			width: cardWidth,
			height: cardHeight,
			depth: cardDepth,
			widthSegments,
			heightSegments,
			depthSegments,
		});

		const positionAttr = geometry.attributes.position;
		const normalAttr = geometry.attributes.normal;
		const positions = positionAttr.data as Float32Array;
		const normals = normalAttr.data as Float32Array;

		const halfW = cardWidth * 0.5;
		const halfH = cardHeight * 0.5;
		const rounded = Math.max(0, Math.min(cardRadius, halfW, halfH));
		const innerW = Math.max(0, halfW - rounded);
		const innerH = Math.max(0, halfH - rounded);

		for (let i = 0; i < positions.length; i += 3) {
			const x = positions[i];
			const y = positions[i + 1];
			const z = positions[i + 2];

			const sx = x < 0 ? -1 : 1;
			const sy = y < 0 ? -1 : 1;

			const ax = Math.abs(x);
			const ay = Math.abs(y);

			const qx = Math.max(ax - innerW, 0);
			const qy = Math.max(ay - innerH, 0);
			const qLen = Math.hypot(qx, qy);

			let nxLocal = 0;
			let nyLocal = 0;

			if (qLen > 1e-6) {
				nxLocal = qx / qLen;
				nyLocal = qy / qLen;
			} else if (ax >= ay) {
				nxLocal = 1;
			} else {
				nyLocal = 1;
			}

			positions[i] = sx * innerW + nxLocal * sx * rounded;
			positions[i + 1] = sy * innerH + nyLocal * sy * rounded;
			positions[i + 2] = z;

			if (Math.abs(normals[i + 2]) > 0.9) {
				normals[i] = 0;
				normals[i + 1] = 0;
				normals[i + 2] = normals[i + 2] > 0 ? 1 : -1;
			} else {
				normals[i] = nxLocal * sx;
				normals[i + 1] = nyLocal * sy;
				normals[i + 2] = 0;
			}
		}

		positionAttr.needsUpdate = true;
		normalAttr.needsUpdate = true;
		return geometry;
	};

	const applyCoverUVMapping = (
		geometry: Box,
		cardWidth: number,
		cardHeight: number,
		imageWidth: number,
		imageHeight: number,
	) => {
		const uvAttr = geometry.attributes.uv;
		const posAttr = geometry.attributes.position;
		const normalAttr = geometry.attributes.normal;
		if (!uvAttr || !posAttr || !normalAttr) return;

		const uvs = uvAttr.data as Float32Array;
		const positions = posAttr.data as Float32Array;
		const normals = normalAttr.data as Float32Array;

		const imageAspect = Math.max(1e-6, imageWidth / imageHeight);
		const cardAspect = Math.max(1e-6, cardWidth / cardHeight);

		let scaleU = 1;
		let scaleV = 1;
		let offsetU = 0;
		let offsetV = 0;

		if (cardAspect > imageAspect) {
			scaleV = imageAspect / cardAspect;
			offsetV = (1 - scaleV) * 0.5;
		} else {
			scaleU = cardAspect / imageAspect;
			offsetU = (1 - scaleU) * 0.5;
		}

		const count = positions.length / 3;
		for (let i = 0; i < count; i++) {
			const ni = i * 3;
			const ui = i * 2;

			if (Math.abs(normals[ni + 2]) <= 0.9) continue;

			const x = positions[ni];
			const y = positions[ni + 1];
			let u = (x + cardWidth * 0.5) / cardWidth;
			let v = (y + cardHeight * 0.5) / cardHeight;

			u = u * scaleU + offsetU;
			v = v * scaleV + offsetV;

			uvs[ui] = u;
			uvs[ui + 1] = v;
		}

		uvAttr.needsUpdate = true;
	};

	$effect(() => {
		if (!setDimensions) return;
		setDimensions({ width, height, depth, radius });
	});

	$effect(() => {
		if (!setImageSource) return;
		setImageSource(image);
	});

	onMount(() => {
		const targetCanvas = canvas;
		if (!targetCanvas) return;

		const renderer = new Renderer({
			canvas: targetCanvas,
			alpha: true,
			antialias: true,
			dpr: typeof window !== "undefined" ? window.devicePixelRatio : 1,
		});
		const gl = renderer.gl;
		gl.clearColor(0, 0, 0, 0);

		targetCanvas.style.width = "100%";
		targetCanvas.style.height = "100%";

		const camera = new Camera(gl, {
			fov: 50,
			aspect: 1,
			near: 0.1,
			far: 100,
		});
		camera.position.set(
			initialCameraPosition.x,
			initialCameraPosition.y,
			initialCameraPosition.z,
		);

		const scene = new Transform();
		const group = new Transform();
		group.setParent(scene);

		const texture = new Texture(gl, {
			image: new Uint8Array([0, 0, 0, 255]),
			width: 1,
			height: 1,
			format: gl.RGBA,
			type: gl.UNSIGNED_BYTE,
			minFilter: gl.LINEAR,
			magFilter: gl.LINEAR,
			wrapS: gl.CLAMP_TO_EDGE,
			wrapT: gl.CLAMP_TO_EDGE,
			generateMipmaps: false,
			flipY: true,
		});

		const vertexShader = `
			precision highp float;

			attribute vec3 position;
			attribute vec2 uv;

			uniform mat4 modelViewMatrix;
			uniform mat4 projectionMatrix;

			varying vec2 vUv;

			void main() {
				vUv = uv;
				gl_Position = projectionMatrix * modelViewMatrix * vec4(position, 1.0);
			}
		`;

		const fragmentShader = `
			precision highp float;

			uniform sampler2D uTexture;
			varying vec2 vUv;

			void main() {
				gl_FragColor = texture2D(uTexture, vUv);
			}
		`;

		const uniforms = {
			uTexture: { value: texture },
		};

		const program = new Program(gl, {
			vertex: vertexShader,
			fragment: fragmentShader,
			uniforms,
			transparent: false,
			depthTest: true,
			depthWrite: true,
		});

		let cardWidth = width;
		let cardHeight = height;
		let cardDepth = depth;
		let cardRadius = radius;
		let imageWidth = 1;
		let imageHeight = 1;

		let geometry = createRoundedCardGeometry(
			gl,
			Math.max(0.001, cardWidth),
			Math.max(0.001, cardHeight),
			Math.max(0.0001, cardDepth),
			Math.max(0, cardRadius),
		);
		applyCoverUVMapping(
			geometry,
			cardWidth,
			cardHeight,
			imageWidth,
			imageHeight,
		);

		const mesh = new Mesh(gl, {
			geometry,
			program,
			frustumCulled: false,
		});
		mesh.setParent(group);

		let imageLoadToken = 0;
		const loadImage = (source: string) => {
			imageLoadToken += 1;
			const token = imageLoadToken;
			const img = new Image();
			img.crossOrigin = "anonymous";
			img.decoding = "async";
			img.onload = () => {
				if (token !== imageLoadToken) return;
				texture.image = img;
				imageWidth = img.naturalWidth || img.width || 1;
				imageHeight = img.naturalHeight || img.height || 1;
				applyCoverUVMapping(
					geometry,
					cardWidth,
					cardHeight,
					imageWidth,
					imageHeight,
				);
			};
			img.src = source;
		};
		setImageSource = loadImage;

		const updateDimensions = (next: {
			width: number;
			height: number;
			depth: number;
			radius: number;
		}) => {
			const nextWidth = Math.max(0.001, next.width);
			const nextHeight = Math.max(0.001, next.height);
			const nextDepth = Math.max(0.0001, next.depth);
			const nextRadius = Math.max(0, next.radius);

			const needsRebuild =
				nextWidth !== cardWidth ||
				nextHeight !== cardHeight ||
				nextDepth !== cardDepth ||
				nextRadius !== cardRadius;

			cardWidth = nextWidth;
			cardHeight = nextHeight;
			cardDepth = nextDepth;
			cardRadius = nextRadius;

			if (!needsRebuild) return;

			const previousGeometry = geometry;
			geometry = createRoundedCardGeometry(
				gl,
				cardWidth,
				cardHeight,
				cardDepth,
				cardRadius,
			);
			applyCoverUVMapping(
				geometry,
				cardWidth,
				cardHeight,
				imageWidth,
				imageHeight,
			);
			mesh.geometry = geometry;
			previousGeometry.remove();
		};
		setDimensions = updateDimensions;

		let raf = 0;
		const tick = () => {
			const w = Math.max(1, targetCanvas.clientWidth);
			const h = Math.max(1, targetCanvas.clientHeight);
			const bufW = Math.round(w * renderer.dpr);
			const bufH = Math.round(h * renderer.dpr);
			if (targetCanvas.width !== bufW || targetCanvas.height !== bufH) {
				targetCanvas.width = bufW;
				targetCanvas.height = bufH;
				renderer.width = w;
				renderer.height = h;
				renderer.state.viewport = { x: 0, y: 0, width: null, height: null };
				camera.perspective({
					fov: 50,
					aspect: w / Math.max(1, h),
					near: 0.1,
					far: 100,
				});
			}
			const targetRotationY = -headPosition.x * 0.5;
			const targetRotationX = headPosition.y * 0.4;

			smoothedRotation.x += (targetRotationX - smoothedRotation.x) * lerpFactor;
			smoothedRotation.y += (targetRotationY - smoothedRotation.y) * lerpFactor;

			group.rotation.x = smoothedRotation.x;
			group.rotation.y = smoothedRotation.y;

			renderer.render({ scene, camera, clear: true });
			raf = window.requestAnimationFrame(tick);
		};

		raf = window.requestAnimationFrame(tick);

		return () => {
			window.cancelAnimationFrame(raf);
			setDimensions = undefined;
			setImageSource = undefined;
			imageLoadToken += 1;

			if (texture.texture) gl.deleteTexture(texture.texture);
			program.remove();
			geometry.remove();
		};
	});
</script>

<canvas
	bind:this={canvas}
	class="absolute inset-0 block h-full w-full"
	style="width:100%;height:100%;"
	aria-hidden="true"
></canvas>
