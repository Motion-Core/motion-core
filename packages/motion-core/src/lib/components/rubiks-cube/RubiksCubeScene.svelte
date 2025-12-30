<script lang="ts">
	import { T, useTask } from "@threlte/core";
	import { OrbitControls } from "@threlte/extras";
	import * as THREE from "three";
	import { RoundedBoxGeometry } from "three/examples/jsm/geometries/RoundedBoxGeometry.js";
	import { SvelteSet } from "svelte/reactivity";

	interface FresnelConfig {
		/**
		 * Base body color for each cubelet.
		 * @default "#111113"
		 */
		color?: THREE.ColorRepresentation;
		/**
		 * Accent color applied by the Fresnel rim.
		 * @default "#FF6900"
		 */
		rimColor?: THREE.ColorRepresentation;
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

	type Cube = {
		id: string;
		position: THREE.Vector3;
		quaternion: THREE.Quaternion;
		originalCoords: { x: number; y: number; z: number };
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

	const initializeCubes = (): Cube[] => {
		const newCubes: Cube[] = [];
		const coords = [-1, 0, 1];
		for (const x of coords) {
			for (const y of coords) {
				for (const z of coords) {
					newCubes.push({
						id: `cube-${x}-${y}-${z}`,
						position: new THREE.Vector3(x, y, z),
						quaternion: new THREE.Quaternion(),
						originalCoords: { x, y, z },
					});
				}
			}
		}
		return newCubes;
	};

	let cubes = $state<Cube[]>(initializeCubes());
	let currentMove = $state<Move | null>(null);

	let mainGroup = $state<THREE.Group>();
	let layerGroup = $state<THREE.Group>();
	let isAnimating = false;
	let currentRotationProgress = 0;
	let lastMoveAxis: Move["axis"] | null = null;
	let timeSinceLastMove = 0;

	const initialCameraPosition = { x: 0, y: 0, z: 10 };

	let geometry = $derived(new RoundedBoxGeometry(size, size, size, 20, radius));

	const vertexShader = `
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
	uniform vec3 color;
	uniform vec3 rimColor;
	uniform float rimPower;
	uniform float rimIntensity;

	varying vec3 vNormal;
	varying vec3 vViewPosition;

	void main() {
		vec3 normal = normalize(vNormal);
		vec3 viewDir = normalize(vViewPosition);

		float rim = 1.0 - max(0.0, dot(normal, viewDir));
		rim = pow(rim, rimPower) * rimIntensity;

		vec3 finalColor = color + rimColor * rim;

		gl_FragColor = vec4(finalColor, 1.0);
        #include <colorspace_fragment>
	}
`;

	const defaultFresnelConfig: Required<FresnelConfig> = {
		color: "#111113",
		rimColor: "#FF6900",
		rimPower: 6,
		rimIntensity: 1.5,
	};

	const material = new THREE.ShaderMaterial({
		vertexShader,
		fragmentShader,
		uniforms: {
			color: { value: new THREE.Color(defaultFresnelConfig.color) },
			rimColor: { value: new THREE.Color(defaultFresnelConfig.rimColor) },
			rimPower: { value: defaultFresnelConfig.rimPower },
			rimIntensity: { value: defaultFresnelConfig.rimIntensity },
		},
	});

	$effect(() => {
		const config = {
			...defaultFresnelConfig,
			...fresnelConfig,
		};

		material.uniforms.color.value.set(config.color);
		material.uniforms.rimColor.value.set(config.rimColor);
		material.uniforms.rimPower.value = config.rimPower;
		material.uniforms.rimIntensity.value = config.rimIntensity;
		material.needsUpdate = true;
	});

	const reusableVec3 = new THREE.Vector3();
	const reusableMatrix4 = new THREE.Matrix4();
	const reusableQuaternion = new THREE.Quaternion();

	const createRotationMatrix = (axis: Move["axis"], angle: number) => {
		reusableMatrix4.identity();
		reusableQuaternion.identity();
		reusableVec3.set(0, 0, 0);
		reusableVec3[axis] = 1;
		reusableQuaternion.setFromAxisAngle(reusableVec3, angle);
		return reusableMatrix4.makeRotationFromQuaternion(reusableQuaternion);
	};

	const commitMove = () => {
		if (!currentMove) return;

		const move = currentMove;
		const angle = (move.rotationAngle || Math.PI / 2) * move.direction;
		const rotMatrix = createRotationMatrix(move.axis, angle);

		const nextCubes = [...cubes];

		nextCubes.forEach((cube) => {
			if (Math.round(cube.position[move.axis]) === move.layer) {
				cube.position.applyMatrix4(rotMatrix);
				const axisVec = reusableVec3.set(
					move.axis === "x" ? 1 : 0,
					move.axis === "y" ? 1 : 0,
					move.axis === "z" ? 1 : 0,
				);
				const deltaQ = new THREE.Quaternion().setFromAxisAngle(axisVec, angle);
				cube.quaternion.premultiply(deltaQ).normalize();
				cube.position.set(
					Math.round(cube.position.x),
					Math.round(cube.position.y),
					Math.round(cube.position.z),
				);
			}
		});

		cubes = nextCubes;
		if (layerGroup) layerGroup.rotation.set(0, 0, 0);
		isAnimating = false;
		currentRotationProgress = 0;
		currentMove = null;
		timeSinceLastMove = 0;
	};

	const beginMove = (move: Move) => {
		if (isAnimating) return;
		currentMove = { ...move, rotationAngle: Math.PI / 2 };
		if (layerGroup) layerGroup.rotation.set(0, 0, 0);
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

	useTask((delta) => {
		if (mainGroup) {
			mainGroup.rotation.x += delta * 0.3;
			mainGroup.rotation.y += delta * 0.5;
			mainGroup.rotation.z += delta * 0.2;
		}

		if (isAnimating && currentMove && layerGroup) {
			const move = currentMove;
			const progressInc = delta / duration;
			currentRotationProgress = Math.min(
				currentRotationProgress + progressInc,
				1,
			);

			const eased = easeInOutCubic(currentRotationProgress);
			const angle =
				eased * (move.rotationAngle || Math.PI / 2) * move.direction;

			if (move.axis === "x") layerGroup.rotation.x = angle;
			else if (move.axis === "y") layerGroup.rotation.y = angle;
			else if (move.axis === "z") layerGroup.rotation.z = angle;

			if (currentRotationProgress >= 1) {
				commitMove();
			}
		} else {
			timeSinceLastMove += delta;
			if (timeSinceLastMove > 0.4) {
				selectNextMove();
			}
		}
	});

	const activeLayerIds = $derived.by(() => {
		const ids = new SvelteSet<string>();
		if (currentMove) {
			cubes.forEach((c) => {
				if (Math.round(c.position[currentMove!.axis]) === currentMove!.layer) {
					ids.add(c.id);
				}
			});
		}
		return ids;
	});

	const staticCubes = $derived(cubes.filter((c) => !activeLayerIds.has(c.id)));
	const activeCubes = $derived(cubes.filter((c) => activeLayerIds.has(c.id)));
</script>

<T.PerspectiveCamera
	makeDefault
	position={[
		initialCameraPosition.x,
		initialCameraPosition.y,
		initialCameraPosition.z,
	]}
>
	<OrbitControls
		enableDamping
		enableZoom={false}
		oncreate={(controls) => {
			controls.target.set(0, 0, 0);
			controls.update();
		}}
	/>
</T.PerspectiveCamera>

<T.Group bind:ref={mainGroup}>
	{#each staticCubes as cube (cube.id)}
		<T.Mesh
			position={[
				cube.position.x * (size + gap),
				cube.position.y * (size + gap),
				cube.position.z * (size + gap),
			]}
			quaternion={[
				cube.quaternion.x,
				cube.quaternion.y,
				cube.quaternion.z,
				cube.quaternion.w,
			]}
			{geometry}
			{material}
		/>
	{/each}

	<T.Group bind:ref={layerGroup}>
		{#each activeCubes as cube (cube.id)}
			<T.Mesh
				position={[
					cube.position.x * (size + gap),
					cube.position.y * (size + gap),
					cube.position.z * (size + gap),
				]}
				quaternion={[
					cube.quaternion.x,
					cube.quaternion.y,
					cube.quaternion.z,
					cube.quaternion.w,
				]}
				{geometry}
				{material}
			/>
		{/each}
	</T.Group>
</T.Group>
