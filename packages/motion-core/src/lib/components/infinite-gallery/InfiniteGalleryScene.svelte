<script lang="ts">
	import { onMount } from "svelte";
	import {
		Camera,
		Mesh,
		Plane,
		Program,
		Raycast,
		Renderer,
		Texture,
		Transform,
		Vec2,
	} from "ogl";

	type ImageItem = string | { src: string; alt?: string };

	interface Props {
		/**
		 * Array of images to display. Can be strings (URL) or objects with src and alt.
		 */
		images: ImageItem[];
		/**
		 * Scroll speed multiplier.
		 * @default 1
		 */
		speed?: number;
		/**
		 * Number of images visible in the tunnel at once.
		 * @default 8
		 */
		visibleCount?: number;
		/**
		 * Configuration for fade in/out effects based on depth.
		 */
		fadeSettings?: {
			fadeIn: { start: number; end: number };
			fadeOut: { start: number; end: number };
		};
		/**
		 * Configuration for blur in/out effects based on depth.
		 */
		blurSettings?: {
			blurIn: { start: number; end: number };
			blurOut: { start: number; end: number };
			maxBlur: number;
		};
	}

	let {
		images,
		speed = 1,
		visibleCount = 8,
		fadeSettings = {
			fadeIn: { start: 0.05, end: 0.15 },
			fadeOut: { start: 0.85, end: 0.95 },
		},
		blurSettings = {
			blurIn: { start: 0.0, end: 0.1 },
			blurOut: { start: 0.9, end: 1.0 },
			maxBlur: 3.0,
		},
	}: Props = $props();

	type NormalizedImage = { src: string; alt?: string };

	type PlaneData = {
		index: number;
		z: number;
		imageIndex: number;
		x: number;
		y: number;
	};

	type PlaneUniforms = {
		map: { value: Texture };
		opacity: { value: number };
		blurAmount: { value: number };
		scrollForce: { value: number };
		time: { value: number };
		isHovered: { value: number };
		uTextureSize: { value: Vec2 };
	};

	type PlaneRuntime = {
		mesh: Mesh;
		program: Program;
		uniforms: PlaneUniforms;
	};

	const DEFAULT_DEPTH_RANGE = 50;
	const MAX_HORIZONTAL_OFFSET = 8;
	const MAX_VERTICAL_OFFSET = 8;

	const normalizeImages = (items: ImageItem[]): NormalizedImage[] =>
		items.map((img) => (typeof img === "string" ? { src: img, alt: "" } : img));

	const makeSpatialPositions = (count: number): { x: number; y: number }[] => {
		const positions: { x: number; y: number }[] = [];

		for (let i = 0; i < count; i++) {
			const horizontalAngle = (i * 2.618) % (Math.PI * 2);
			const verticalAngle = (i * 1.618 + Math.PI / 3) % (Math.PI * 2);

			const horizontalRadius = (i % 3) * 1.2;
			const verticalRadius = ((i + 1) % 4) * 0.8;

			const x =
				(Math.sin(horizontalAngle) * horizontalRadius * MAX_HORIZONTAL_OFFSET) /
				3;
			const y =
				(Math.cos(verticalAngle) * verticalRadius * MAX_VERTICAL_OFFSET) / 4;

			positions.push({ x, y });
		}

		return positions;
	};

	const vertexShader = `
		attribute vec3 position;
		attribute vec3 normal;
		attribute vec2 uv;

		uniform mat4 modelViewMatrix;
		uniform mat4 projectionMatrix;
		uniform float scrollForce;
		uniform float time;
		uniform float isHovered;

		varying vec2 vUv;
		varying vec3 vNormal;

		void main() {
			vUv = uv;
			vNormal = normal;

			vec3 pos = position;

			float curveIntensity = scrollForce * 0.3;
			float distanceFromCenter = length(pos.xy);
			float curve = distanceFromCenter * distanceFromCenter * curveIntensity;

			float ripple1 = sin(pos.x * 2.0 + scrollForce * 3.0) * 0.02;
			float ripple2 = sin(pos.y * 2.5 + scrollForce * 2.0) * 0.015;
			float clothEffect = (ripple1 + ripple2) * abs(curveIntensity) * 2.0;

			float flagWave = 0.0;
			if (isHovered > 0.5) {
				float wavePhase = pos.x * 3.0 + time * 8.0;
				float waveAmplitude = sin(wavePhase) * 0.1;
				float dampening = smoothstep(-0.5, 0.5, pos.x);
				flagWave = waveAmplitude * dampening;

				float secondaryWave = sin(pos.x * 5.0 + time * 12.0) * 0.03 * dampening;
				flagWave += secondaryWave;
			}

			pos.z -= (curve + clothEffect + flagWave);

			gl_Position = projectionMatrix * modelViewMatrix * vec4(pos, 1.0);
		}
	`;

	const fragmentShader = `
		precision highp float;

		uniform sampler2D map;
		uniform float opacity;
		uniform float blurAmount;
		uniform float scrollForce;
		uniform vec2 uTextureSize;

		varying vec2 vUv;
		varying vec3 vNormal;

		void main() {
			vec4 color = texture2D(map, vUv);

			if (blurAmount > 0.0) {
				vec2 texelSize = 1.0 / max(uTextureSize, vec2(1.0));
				vec4 blurred = vec4(0.0);
				float total = 0.0;

				for (float x = -2.0; x <= 2.0; x += 1.0) {
					for (float y = -2.0; y <= 2.0; y += 1.0) {
						vec2 offset = vec2(x, y) * texelSize * blurAmount;
						float weight = 1.0 / (1.0 + length(vec2(x, y)));
						blurred += texture2D(map, vUv + offset) * weight;
						total += weight;
					}
				}
				color = blurred / total;
			}

			float curveHighlight = abs(scrollForce) * 0.05;
			color.rgb += vec3(curveHighlight * 0.1);

			gl_FragColor = vec4(color.rgb, color.a * opacity);
		}
	`;

	let canvas = $state<HTMLCanvasElement>();
	let setImageItems = $state<(items: ImageItem[]) => void>();

	$effect(() => {
		if (!setImageItems) return;
		setImageItems(images);
	});

	onMount(() => {
		const targetCanvas = canvas;
		if (!targetCanvas) return;

		const count = Math.max(1, Math.floor(visibleCount));
		const depthRange = DEFAULT_DEPTH_RANGE;
		const totalRange = depthRange;
		const spatialPositions = makeSpatialPositions(count);

		const renderer = new Renderer({
			canvas: targetCanvas,
			alpha: true,
			antialias: true,
			dpr: typeof window !== "undefined" ? window.devicePixelRatio : 1,
		});
		const gl = renderer.gl;
		gl.clearColor(0, 0, 0, 0);

		const camera = new Camera(gl, {
			fov: 55,
			aspect: 1,
			near: 0.1,
			far: 100,
		});
		camera.position.set(0, 0, 0);

		const scene = new Transform();
		const geometry = new Plane(gl, {
			width: 1,
			height: 1,
			widthSegments: 32,
			heightSegments: 32,
		});

		const fallbackTexture = new Texture(gl, {
			image: new Uint8Array([0, 0, 0, 255]),
			width: 1,
			height: 1,
			format: gl.RGBA,
			type: gl.UNSIGNED_BYTE,
			minFilter: gl.LINEAR,
			magFilter: gl.LINEAR,
			wrapS: gl.MIRRORED_REPEAT,
			wrapT: gl.MIRRORED_REPEAT,
			generateMipmaps: false,
			flipY: true,
			anisotropy: renderer.parameters.maxAnisotropy,
		});

		let normalizedImages = normalizeImages(images);
		let textures: Texture[] = [];
		let imageLoadToken = 0;
		let disposed = false;

		const disposeTexture = (texture: Texture) => {
			if (texture.texture) {
				gl.deleteTexture(texture.texture);
			}
		};

		const setTexturesFromImages = (items: ImageItem[]) => {
			normalizedImages = normalizeImages(items);
			imageLoadToken += 1;
			const token = imageLoadToken;

			textures.forEach(disposeTexture);
			textures = [];

			for (let i = 0; i < normalizedImages.length; i++) {
				const texture = new Texture(gl, {
					image: new Uint8Array([0, 0, 0, 255]),
					width: 1,
					height: 1,
					format: gl.RGBA,
					type: gl.UNSIGNED_BYTE,
					minFilter: gl.LINEAR,
					magFilter: gl.LINEAR,
					wrapS: gl.MIRRORED_REPEAT,
					wrapT: gl.MIRRORED_REPEAT,
					generateMipmaps: false,
					flipY: true,
					anisotropy: renderer.parameters.maxAnisotropy,
				});
				textures.push(texture);

				const img = new Image();
				img.crossOrigin = "anonymous";
				img.decoding = "async";
				img.onload = () => {
					if (disposed || token !== imageLoadToken) return;
					texture.image = img;
				};
				img.src = normalizedImages[i].src;
			}
		};
		setImageItems = setTexturesFromImages;

		const planesData: PlaneData[] = Array.from({ length: count }, (_, i) => ({
			index: i,
			z: count > 0 ? ((depthRange / count) * i) % depthRange : 0,
			imageIndex: normalizedImages.length > 0 ? i % normalizedImages.length : 0,
			x: spatialPositions[i]?.x ?? 0,
			y: spatialPositions[i]?.y ?? 0,
		}));

		const planes: PlaneRuntime[] = Array.from({ length: count }, () => {
			const uniforms: PlaneUniforms = {
				map: { value: fallbackTexture },
				opacity: { value: 1 },
				blurAmount: { value: 0 },
				scrollForce: { value: 0 },
				time: { value: 0 },
				isHovered: { value: 0 },
				uTextureSize: { value: new Vec2(1, 1) },
			};

			const program = new Program(gl, {
				vertex: vertexShader,
				fragment: fragmentShader,
				uniforms,
				transparent: true,
				depthTest: true,
				depthWrite: true,
			});

			const mesh = new Mesh(gl, {
				geometry,
				program,
				frustumCulled: false,
			});
			mesh.setParent(scene);

			return { mesh, program, uniforms };
		});

		let scrollVelocity = 0;
		let autoPlay = true;
		let lastInteraction = Date.now();
		let elapsedTime = 0;

		const handleWheel = (event: WheelEvent) => {
			event.preventDefault();
			scrollVelocity += event.deltaY * 0.01 * speed;
			autoPlay = false;
			lastInteraction = Date.now();
		};

		const handleKeyDown = (event: KeyboardEvent) => {
			if (event.key === "ArrowUp" || event.key === "ArrowLeft") {
				scrollVelocity -= 2 * speed;
				autoPlay = false;
				lastInteraction = Date.now();
			} else if (event.key === "ArrowDown" || event.key === "ArrowRight") {
				scrollVelocity += 2 * speed;
				autoPlay = false;
				lastInteraction = Date.now();
			}
		};

		targetCanvas.addEventListener("wheel", handleWheel, { passive: false });
		window.addEventListener("keydown", handleKeyDown);

		const autoPlayInterval = window.setInterval(() => {
			if (Date.now() - lastInteraction > 3000) {
				autoPlay = true;
			}
		}, 1000);

		const raycast = new Raycast();
		const pointer = new Vec2(0, 0);
		let pointerActive = false;
		let hoveredIndex = -1;
		const meshIdToIndex: Record<number, number> = {};
		planes.forEach((plane, index) => {
			meshIdToIndex[plane.mesh.id] = index;
		});

		const handlePointerMove = (event: PointerEvent) => {
			const rect = targetCanvas.getBoundingClientRect();
			if (rect.width <= 0 || rect.height <= 0) return;
			pointer.x = ((event.clientX - rect.left) / rect.width) * 2 - 1;
			pointer.y = -(((event.clientY - rect.top) / rect.height) * 2 - 1);
			pointerActive = true;
		};

		const handlePointerLeave = () => {
			pointerActive = false;
			if (hoveredIndex !== -1) {
				hoveredIndex = -1;
				for (let i = 0; i < planes.length; i++) {
					planes[i].uniforms.isHovered.value = 0;
				}
			}
		};

		targetCanvas.addEventListener("pointermove", handlePointerMove);
		targetCanvas.addEventListener("pointerleave", handlePointerLeave);

		const resize = () => {
			const host = targetCanvas.parentElement ?? targetCanvas;
			const { width: hostWidth, height: hostHeight } =
				host.getBoundingClientRect();
			const width = Math.max(1, Math.round(hostWidth));
			const height = Math.max(1, Math.round(hostHeight));

			renderer.setSize(width, height);
			camera.perspective({
				fov: 55,
				aspect: width / Math.max(1, height),
				near: 0.1,
				far: 100,
			});

			for (let i = 0; i < planes.length; i++) {
				planes[i].uniforms.scrollForce.value = scrollVelocity;
			}
		};

		resize();
		const observer = new ResizeObserver(resize);
		observer.observe(targetCanvas);
		if (targetCanvas.parentElement)
			observer.observe(targetCanvas.parentElement);

		let raf = 0;
		let previous = 0;
		const tick = (now: number) => {
			const delta = previous ? (now - previous) / 1000 : 0;
			previous = now;
			elapsedTime += delta;

			if (autoPlay) {
				scrollVelocity += 0.3 * delta;
			}

			scrollVelocity *= 0.95;

			const totalImages = normalizedImages.length;
			const imageAdvance =
				totalImages > 0 ? count % totalImages || totalImages : 0;

			for (let i = 0; i < planesData.length; i++) {
				const planeData = planesData[i];
				const plane = planes[i];

				plane.uniforms.time.value = elapsedTime;
				plane.uniforms.scrollForce.value = scrollVelocity;

				let newZ = planeData.z + scrollVelocity * delta * 10;
				let wrapsForward = 0;
				let wrapsBackward = 0;

				if (newZ >= totalRange) {
					wrapsForward = Math.floor(newZ / totalRange);
					newZ -= totalRange * wrapsForward;
				} else if (newZ < 0) {
					wrapsBackward = Math.ceil(-newZ / totalRange);
					newZ += totalRange * wrapsBackward;
				}

				if (wrapsForward > 0 && imageAdvance > 0 && totalImages > 0) {
					planeData.imageIndex =
						(planeData.imageIndex + wrapsForward * imageAdvance) % totalImages;
				}

				if (wrapsBackward > 0 && imageAdvance > 0 && totalImages > 0) {
					const step = planeData.imageIndex - wrapsBackward * imageAdvance;
					planeData.imageIndex =
						((step % totalImages) + totalImages) % totalImages;
				}

				planeData.z = ((newZ % totalRange) + totalRange) % totalRange;
				planeData.x = spatialPositions[i]?.x ?? 0;
				planeData.y = spatialPositions[i]?.y ?? 0;

				const normalizedPosition = planeData.z / totalRange;
				let opacity = 1;

				if (
					normalizedPosition >= fadeSettings.fadeIn.start &&
					normalizedPosition <= fadeSettings.fadeIn.end
				) {
					const fadeInProgress =
						(normalizedPosition - fadeSettings.fadeIn.start) /
						(fadeSettings.fadeIn.end - fadeSettings.fadeIn.start);
					opacity = fadeInProgress;
				} else if (normalizedPosition < fadeSettings.fadeIn.start) {
					opacity = 0;
				} else if (
					normalizedPosition >= fadeSettings.fadeOut.start &&
					normalizedPosition <= fadeSettings.fadeOut.end
				) {
					const fadeOutProgress =
						(normalizedPosition - fadeSettings.fadeOut.start) /
						(fadeSettings.fadeOut.end - fadeSettings.fadeOut.start);
					opacity = 1 - fadeOutProgress;
				} else if (normalizedPosition > fadeSettings.fadeOut.end) {
					opacity = 0;
				}

				opacity = Math.max(0, Math.min(1, opacity));

				let blur = 0;

				if (
					normalizedPosition >= blurSettings.blurIn.start &&
					normalizedPosition <= blurSettings.blurIn.end
				) {
					const blurInProgress =
						(normalizedPosition - blurSettings.blurIn.start) /
						(blurSettings.blurIn.end - blurSettings.blurIn.start);
					blur = blurSettings.maxBlur * (1 - blurInProgress);
				} else if (normalizedPosition < blurSettings.blurIn.start) {
					blur = blurSettings.maxBlur;
				} else if (
					normalizedPosition >= blurSettings.blurOut.start &&
					normalizedPosition <= blurSettings.blurOut.end
				) {
					const blurOutProgress =
						(normalizedPosition - blurSettings.blurOut.start) /
						(blurSettings.blurOut.end - blurSettings.blurOut.start);
					blur = blurSettings.maxBlur * blurOutProgress;
				} else if (normalizedPosition > blurSettings.blurOut.end) {
					blur = blurSettings.maxBlur;
				}

				blur = Math.max(0, Math.min(blurSettings.maxBlur, blur));

				plane.uniforms.opacity.value = opacity;
				plane.uniforms.blurAmount.value = blur;

				const texture =
					totalImages > 0
						? (textures[planeData.imageIndex] ?? fallbackTexture)
						: fallbackTexture;
				plane.uniforms.map.value = texture;

				const texWidth =
					texture.image && "width" in texture.image
						? Math.max(1, Number(texture.image.width) || 1)
						: 1;
				const texHeight =
					texture.image && "height" in texture.image
						? Math.max(1, Number(texture.image.height) || 1)
						: 1;
				plane.uniforms.uTextureSize.value.set(texWidth, texHeight);

				const aspect = texWidth / texHeight;
				if (aspect > 1) {
					plane.mesh.scale.set(2 * aspect, 2, 1);
				} else {
					plane.mesh.scale.set(2, 2 / Math.max(aspect, 0.00001), 1);
				}

				const worldZ = planeData.z - depthRange / 2;
				plane.mesh.position.set(planeData.x, planeData.y, worldZ);
			}

			if (pointerActive) {
				raycast.castMouse(camera, [pointer.x, pointer.y]);
				const hits = raycast.intersectMeshes(
					planes.map((plane) => plane.mesh),
					{ includeUV: false, includeNormal: false },
				);
				const nextHover =
					hits.length > 0 ? (meshIdToIndex[hits[0].id] ?? -1) : -1;

				if (nextHover !== hoveredIndex) {
					hoveredIndex = nextHover;
					for (let i = 0; i < planes.length; i++) {
						planes[i].uniforms.isHovered.value = i === hoveredIndex ? 1 : 0;
					}
				}
			}

			renderer.render({ scene, camera, clear: true });
			raf = window.requestAnimationFrame(tick);
		};

		raf = window.requestAnimationFrame(tick);

		return () => {
			disposed = true;
			imageLoadToken += 1;
			window.cancelAnimationFrame(raf);
			window.clearInterval(autoPlayInterval);
			observer.disconnect();
			targetCanvas.removeEventListener("wheel", handleWheel);
			window.removeEventListener("keydown", handleKeyDown);
			targetCanvas.removeEventListener("pointermove", handlePointerMove);
			targetCanvas.removeEventListener("pointerleave", handlePointerLeave);
			setImageItems = undefined;

			for (let i = 0; i < planes.length; i++) {
				planes[i].program.remove();
			}
			geometry.remove();
			textures.forEach(disposeTexture);
			disposeTexture(fallbackTexture);
		};
	});
</script>

<canvas
	bind:this={canvas}
	class="absolute inset-0 block h-full w-full"
	style="width:100%;height:100%;"
	aria-hidden="true"
></canvas>
