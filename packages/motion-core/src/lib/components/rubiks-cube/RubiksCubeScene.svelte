<script lang="ts">
	import { onMount } from "svelte";
	import { SvelteSet } from "svelte/reactivity";
	import {
		Box,
		Camera,
		Mat4,
		Mesh,
		Orbit,
		Program,
		Quat,
		Renderer,
		Transform,
		Vec3,
	} from "ogl";
	import { type ColorRepresentation, toLinearRgb } from "../../helpers/color";

	interface FresnelConfig {
		/**
		 * Base body color for each cubelet.
		 * @default "#17181A"
		 */
		color?: ColorRepresentation;
		/**
		 * Accent color applied by the Fresnel rim.
		 * @default "#FF6900"
		 */
		rimColor?: ColorRepresentation;
		/**
		 * Controls how tight the Fresnel rim hug is.
		 * Higher values yield a thinner outline.
		 * @default 6
		 */
		rimPower?: number;
		/**
		 * Intensity multiplier for the Fresnel rim color.
		 * @default 1.5
		 */
		rimIntensity?: number;
	}

	interface Props {
		/**
		 * Size of an individual cubelet edge.
		 * @default 1
		 */
		size: number;
		/**
		 * Seconds it takes to complete a face rotation.
		 * @default 1.5
		 */
		duration: number;
		/**
		 * Gap between cubelets to accentuate separation.
		 * @default 0.015
		 */
		gap: number;
		/**
		 * Corner radius for softened cube edges.
		 * @default 0.125
		 */
		radius: number;
		/**
		 * Optional overrides for the Fresnel shader uniforms.
		 */
		fresnelConfig?: FresnelConfig;
	}

	let { size, duration, gap, radius, fresnelConfig = {} }: Props = $props();

	type Move = {
		axis: "x" | "y" | "z";
		layer: -1 | 0 | 1;
		direction: 1 | -1;
		rotationAngle?: number;
	};

	type CubeState = {
		id: string;
		position: Vec3;
		quaternion: Quat;
		mesh: Mesh;
	};

	const POSSIBLE_MOVES: Move[] = (() => {
		const moves: Move[] = [];
		for (const axis of ["x", "y", "z"] as const) {
			for (const layer of [-1, 0, 1] as const) {
				for (const direction of [1, -1] as const) {
					moves.push({ axis, layer, direction });
				}
			}
		}
		return moves;
	})();

	const easeInOutCubic = (t: number) =>
		t < 0.5 ? 4 * t * t * t : 1 - Math.pow(-2 * t + 2, 3) / 2;

	const defaultFresnelConfig: Required<FresnelConfig> = {
		color: "#17181A",
		rimColor: "#FF6900",
		rimPower: 6,
		rimIntensity: 1.5,
	};

	const createRoundedBoxGeometry = (
		gl: Renderer["gl"],
		cubeSize: number,
		cubeRadius: number,
	) => {
		const segments = 20;
		const geometry = new Box(gl, {
			width: cubeSize,
			height: cubeSize,
			depth: cubeSize,
			widthSegments: segments,
			heightSegments: segments,
			depthSegments: segments,
		});

		const positionAttr = geometry.attributes.position;
		const normalAttr = geometry.attributes.normal;
		const positions = positionAttr.data as Float32Array;
		const normals = normalAttr.data as Float32Array;

		const half = cubeSize * 0.5;
		const rounded = Math.max(0, Math.min(cubeRadius, half));
		const inner = Math.max(0, half - rounded);

		for (let i = 0; i < positions.length; i += 3) {
			const x = positions[i];
			const y = positions[i + 1];
			const z = positions[i + 2];

			const sx = x < 0 ? -1 : 1;
			const sy = y < 0 ? -1 : 1;
			const sz = z < 0 ? -1 : 1;

			const ax = Math.abs(x);
			const ay = Math.abs(y);
			const az = Math.abs(z);

			const qx = Math.max(ax - inner, 0);
			const qy = Math.max(ay - inner, 0);
			const qz = Math.max(az - inner, 0);
			const qLen = Math.hypot(qx, qy, qz);

			let nx = 0;
			let ny = 0;
			let nz = 0;

			if (qLen > 1e-6) {
				nx = qx / qLen;
				ny = qy / qLen;
				nz = qz / qLen;
			} else {
				if (ax >= ay && ax >= az) nx = 1;
				else if (ay >= ax && ay >= az) ny = 1;
				else nz = 1;
			}

			normals[i] = nx * sx;
			normals[i + 1] = ny * sy;
			normals[i + 2] = nz * sz;

			positions[i] = sx * inner + nx * sx * rounded;
			positions[i + 1] = sy * inner + ny * sy * rounded;
			positions[i + 2] = sz * inner + nz * sz * rounded;
		}

		positionAttr.needsUpdate = true;
		normalAttr.needsUpdate = true;
		return geometry;
	};

	let canvas = $state<HTMLCanvasElement>();
	let setDimensions =
		$state<(next: { size: number; gap: number; radius: number }) => void>();
	let setFresnelUniforms = $state<(config: FresnelConfig) => void>();

	$effect(() => {
		if (!setDimensions) return;
		setDimensions({ size, gap, radius });
	});

	$effect(() => {
		if (!setFresnelUniforms) return;
		setFresnelUniforms(fresnelConfig ?? {});
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
			fov: 55,
			aspect: 1,
			near: 0.1,
			far: 100,
		});
		camera.position.set(0, 0, 10);

		const scene = new Transform();
		const mainGroup = new Transform();
		mainGroup.setParent(scene);
		const layerGroup = new Transform();
		layerGroup.setParent(mainGroup);

		const orbit = new Orbit(camera, {
			element: targetCanvas,
			enableZoom: false,
			target: new Vec3(0, 0, 0),
			ease: 0.15,
			inertia: 0.85,
		});

		let cubeSize = size;
		let cubeGap = gap;
		let cubeRadius = radius;

		let geometry = createRoundedBoxGeometry(gl, cubeSize, cubeRadius);

		const uniforms = {
			color: { value: new Vec3(17 / 255, 17 / 255, 19 / 255) },
			rimColor: { value: new Vec3(1, 105 / 255, 0) },
			rimPower: { value: 6 },
			rimIntensity: { value: 1.5 },
		};

		const vertexShader = `
			precision highp float;

			attribute vec3 position;
			attribute vec3 normal;

			uniform mat4 modelViewMatrix;
			uniform mat4 projectionMatrix;
			uniform mat3 normalMatrix;

			varying vec3 vNormal;
			varying vec3 vViewPosition;

			void main() {
				vNormal = normalize(normalMatrix * normal);
				vec4 mvPosition = modelViewMatrix * vec4(position, 1.0);
				vViewPosition = -mvPosition.xyz;
				gl_Position = projectionMatrix * mvPosition;
			}
		`;

		const fragmentShader = `
			precision highp float;

			uniform vec3 color;
			uniform vec3 rimColor;
			uniform float rimPower;
			uniform float rimIntensity;

			varying vec3 vNormal;
			varying vec3 vViewPosition;

			vec3 linearToSrgb(vec3 color) {
				vec3 safe = max(color, vec3(0.0));
				vec3 low = safe * 12.92;
				vec3 high = 1.055 * pow(safe, vec3(1.0 / 2.4)) - 0.055;
				vec3 cutoff = step(vec3(0.0031308), safe);
				return mix(low, high, cutoff);
			}

			void main() {
				vec3 normal = normalize(vNormal);
				vec3 viewDir = normalize(vViewPosition);
				float rim = 1.0 - max(0.0, dot(normal, viewDir));
				rim = pow(rim, rimPower) * rimIntensity;
				vec3 finalColor = color + rimColor * rim;
				gl_FragColor = vec4(linearToSrgb(finalColor), 1.0);
			}
		`;

		const material = new Program(gl, {
			vertex: vertexShader,
			fragment: fragmentShader,
			uniforms,
			transparent: false,
			depthTest: true,
			depthWrite: true,
		});

		const coords = [-1, 0, 1];
		const cubes: CubeState[] = [];
		for (const x of coords) {
			for (const y of coords) {
				for (const z of coords) {
					const mesh = new Mesh(gl, {
						geometry,
						program: material,
						frustumCulled: false,
					});
					mesh.setParent(mainGroup);

					const cube: CubeState = {
						id: `cube-${x}-${y}-${z}`,
						position: new Vec3(x, y, z),
						quaternion: new Quat(),
						mesh,
					};
					cubes.push(cube);
				}
			}
		}

		const updateCubeTransform = (cube: CubeState) => {
			const spacing = cubeSize + cubeGap;
			cube.mesh.position.set(
				cube.position.x * spacing,
				cube.position.y * spacing,
				cube.position.z * spacing,
			);
			cube.mesh.quaternion.copy(cube.quaternion);
		};

		for (let i = 0; i < cubes.length; i++) updateCubeTransform(cubes[i]);

		let activeLayerSet = new SvelteSet<string>();
		let currentMove: Move | null = null;
		let isAnimating = false;
		let currentRotationProgress = 0;
		let lastMoveAxis: Move["axis"] | null = null;
		let timeSinceLastMove = 0;

		const rotationMatrix = new Mat4();
		const tempQuat = new Quat();
		const deltaQuat = new Quat();
		const axisVec = new Vec3();

		const createRotationMatrix = (axis: Move["axis"], angle: number) => {
			axisVec.set(
				axis === "x" ? 1 : 0,
				axis === "y" ? 1 : 0,
				axis === "z" ? 1 : 0,
			);
			tempQuat.fromAxisAngle(axisVec, angle);
			rotationMatrix.identity().fromQuaternion(tempQuat);
			return rotationMatrix;
		};

		const resetLayerGrouping = () => {
			for (let i = 0; i < cubes.length; i++) {
				cubes[i].mesh.setParent(mainGroup);
			}
			activeLayerSet = new SvelteSet();
			layerGroup.rotation.set(0, 0, 0);
		};

		const selectActiveLayer = (move: Move) => {
			activeLayerSet = new SvelteSet();
			for (let i = 0; i < cubes.length; i++) {
				const cube = cubes[i];
				if (Math.round(cube.position[move.axis]) === move.layer) {
					activeLayerSet.add(cube.id);
					cube.mesh.setParent(layerGroup);
				} else {
					cube.mesh.setParent(mainGroup);
				}
			}
			layerGroup.rotation.set(0, 0, 0);
		};

		const commitMove = () => {
			if (!currentMove) return;

			const move = currentMove;
			const angle = (move.rotationAngle || Math.PI / 2) * move.direction;
			const matrix = createRotationMatrix(move.axis, angle);
			axisVec.set(
				move.axis === "x" ? 1 : 0,
				move.axis === "y" ? 1 : 0,
				move.axis === "z" ? 1 : 0,
			);
			deltaQuat.fromAxisAngle(axisVec, angle);

			for (let i = 0; i < cubes.length; i++) {
				const cube = cubes[i];
				if (!activeLayerSet.has(cube.id)) continue;

				cube.position.applyMatrix4(matrix);
				cube.position.set(
					Math.round(cube.position.x),
					Math.round(cube.position.y),
					Math.round(cube.position.z),
				);

				tempQuat.multiply(deltaQuat, cube.quaternion);
				cube.quaternion.copy(tempQuat).normalize();
				updateCubeTransform(cube);
			}

			resetLayerGrouping();
			isAnimating = false;
			currentRotationProgress = 0;
			currentMove = null;
			timeSinceLastMove = 0;
		};

		const beginMove = (move: Move) => {
			if (isAnimating) return;
			currentMove = { ...move, rotationAngle: Math.PI / 2 };
			selectActiveLayer(currentMove);
			isAnimating = true;
			currentRotationProgress = 0;
			lastMoveAxis = move.axis;
		};

		const selectNextMove = () => {
			const moves = POSSIBLE_MOVES.filter((m) => m.axis !== lastMoveAxis);
			if (moves.length === 0) return;
			const move = moves[Math.floor(Math.random() * moves.length)];
			beginMove(move);
		};

		const applyFresnelConfig = (config: FresnelConfig) => {
			const next = {
				...defaultFresnelConfig,
				...config,
			};
			const [cr, cg, cb] = toLinearRgb(next.color, [
				17 / 255,
				17 / 255,
				19 / 255,
			]);
			const [rr, rg, rb] = toLinearRgb(next.rimColor, [1, 105 / 255, 0]);
			uniforms.color.value.set(cr, cg, cb);
			uniforms.rimColor.value.set(rr, rg, rb);
			uniforms.rimPower.value = next.rimPower;
			uniforms.rimIntensity.value = next.rimIntensity;
		};
		setFresnelUniforms = applyFresnelConfig;

		const applyDimensions = (next: {
			size: number;
			gap: number;
			radius: number;
		}) => {
			const nextSize = Math.max(0.0001, next.size);
			const nextGap = Math.max(0, next.gap);
			const nextRadius = Math.max(0, next.radius);

			const shouldRebuild =
				nextSize !== cubeSize || Math.abs(nextRadius - cubeRadius) > 1e-6;

			cubeSize = nextSize;
			cubeGap = nextGap;
			cubeRadius = nextRadius;

			if (shouldRebuild) {
				const prev = geometry;
				geometry = createRoundedBoxGeometry(gl, cubeSize, cubeRadius);
				for (let i = 0; i < cubes.length; i++) {
					cubes[i].mesh.geometry = geometry;
				}
				prev.remove();
			}

			for (let i = 0; i < cubes.length; i++) {
				updateCubeTransform(cubes[i]);
			}
		};
		setDimensions = applyDimensions;

		let raf = 0;
		let previous = 0;
		const tick = (now: number) => {
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
					fov: 55,
					aspect: w / Math.max(1, h),
					near: 0.1,
					far: 100,
				});
			}
			const delta = previous ? (now - previous) / 1000 : 0;
			previous = now;

			mainGroup.rotation.x += delta * 0.3;
			mainGroup.rotation.y += delta * 0.5;
			mainGroup.rotation.z += delta * 0.2;

			orbit.update();

			if (isAnimating && currentMove) {
				const progressInc = delta / Math.max(0.0001, duration);
				currentRotationProgress = Math.min(
					currentRotationProgress + progressInc,
					1,
				);

				const eased = easeInOutCubic(currentRotationProgress);
				const angle =
					eased *
					(currentMove.rotationAngle || Math.PI / 2) *
					currentMove.direction;

				if (currentMove.axis === "x") layerGroup.rotation.x = angle;
				else if (currentMove.axis === "y") layerGroup.rotation.y = angle;
				else layerGroup.rotation.z = angle;

				if (currentRotationProgress >= 1) {
					commitMove();
				}
			} else {
				timeSinceLastMove += delta;
				if (timeSinceLastMove > 0.4) {
					selectNextMove();
				}
			}

			renderer.render({ scene, camera, clear: true });
			raf = window.requestAnimationFrame(tick);
		};

		raf = window.requestAnimationFrame(tick);

		return () => {
			window.cancelAnimationFrame(raf);
			orbit.remove();
			setDimensions = undefined;
			setFresnelUniforms = undefined;

			material.remove();
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
