<script lang="ts">
	import { T, useThrelte } from "@threlte/core";
	import { OrbitControls, interactivity } from "@threlte/extras";
	import * as THREE from "three";
	import type { OrbitControls as OrbitControlsType } from "three/examples/jsm/controls/OrbitControls.js";
	import { gsap } from "gsap/dist/gsap";
	import type { Snippet } from "svelte";
	import landTextureUrl from "../../assets/land-texture.png";
	import type { GlobeMarker, GlobeMarkerTooltipContext } from "./types";
	import GlobeMarkerItem from "./GlobeMarkerItem.svelte";

	interactivity();

	interface FresnelConfig {
		/**
		 * Base body color for the globe surface.
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

	interface AtmosphereConfig {
		/**
		 * Color of the atmosphere glow.
		 * @default "#FF6900"
		 */
		color?: THREE.ColorRepresentation;
		/**
		 * Size of the atmosphere relative to the globe radius.
		 * @default 1.1
		 */
		scale?: number;
		/**
		 * Falloff power of the glow. Higher values mean a sharper edge.
		 * @default 12.0
		 */
		power?: number;
		/**
		 * Base coefficient for the intensity calculation.
		 * Controls how far the glow extends inwards.
		 * @default 0.9
		 */
		coefficient?: number;
		/**
		 * Global intensity multiplier.
		 * @default 2.0
		 */
		intensity?: number;
	}

	interface Props {
		/**
		 * Radius of the sphere.
		 * @default 2
		 */
		radius: number;
		/**
		 * Optional overrides for the Fresnel shader uniforms.
		 */
		fresnelConfig?: FresnelConfig;
		/**
		 * Optional configuration for the atmospheric halo.
		 */
		atmosphereConfig?: AtmosphereConfig;
		/**
		 * Number of points rendered along the globe surface.
		 * @default 15000
		 */
		pointCount?: number;
		/**
		 * Size of each point in world units.
		 * @default 0.05
		 */
		pointSize?: number;
		/**
		 * Color applied to points representing land.
		 * @default "#f77114"
		 */
		landPointColor?: THREE.ColorRepresentation;
		/**
		 * Whether the globe should auto-rotate.
		 * @default true
		 */
		autoRotate?: boolean;
		/**
		 * Whether to lock the camera's polar angle.
		 * @default true
		 */
		lockedPolarAngle?: boolean;
		/**
		 * Markers to display on the globe.
		 */
		markers?: GlobeMarker[];
		/**
		 * Optional custom tooltip renderer for markers.
		 */
		markerTooltip?: Snippet<[GlobeMarkerTooltipContext]>;
		/**
		 * Coordinates [lat, lon] to focus on.
		 */
		focusOn?: [number, number] | null;
	}

	interface LandMaskData {
		width: number;
		height: number;
		data: Uint8ClampedArray;
	}

	const DEG2RAD = Math.PI / 180;
	const EPSILON = 1e-9;
	const LAND_MASK_THRESHOLD = 0.5;

	let {
		radius,
		fresnelConfig = {},
		atmosphereConfig = {},
		pointCount = 15000,
		pointSize = 0.05,
		landPointColor = "#f77114",
		autoRotate = true,
		lockedPolarAngle = true,
		markers = [],
		markerTooltip,
		focusOn = null,
	}: Props = $props();

	const initialCameraPosition = { x: 0, y: 0, z: 8 };
	let globeGroup = $state<THREE.Group>();
	let controls = $state<OrbitControlsType>();
	let focusTween: gsap.core.Tween | null = null;
	let landMask = $state<LandMaskData | null>(null);

	const { camera } = useThrelte();

	const SEGMENTS = 64;

	let geometry = $derived(new THREE.SphereGeometry(radius, SEGMENTS, SEGMENTS));

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

	const atmosphereVertexShader = `
	varying vec3 vNormal;
	void main() {
		vNormal = normalize(normalMatrix * normal);
		gl_Position = projectionMatrix * modelViewMatrix * vec4(position, 1.0);
	}
`;

	const atmosphereFragmentShader = `
	uniform vec3 color;
	uniform float power;
	uniform float coefficient;
	uniform float intensity;

	varying vec3 vNormal;

	void main() {
		vec3 viewDir = vec3(0.0, 0.0, 1.0);
		float viewDot = dot(vNormal, viewDir);

		float factor = pow(max(0.0, coefficient - viewDot), power);

		vec3 finalColor = color * factor * intensity;

		gl_FragColor = vec4(finalColor, factor * intensity);
		#include <colorspace_fragment>
	}
`;

	const defaultFresnelConfig: Required<FresnelConfig> = {
		color: "#111113",
		rimColor: "#FF6900",
		rimPower: 6,
		rimIntensity: 1.5,
	};

	const defaultAtmosphereConfig: Required<AtmosphereConfig> = {
		color: "#FF6900",
		scale: 1.1,
		power: 12.0,
		coefficient: 0.9,
		intensity: 2.0,
	};

	const resolvedFresnelConfig = $derived({
		...defaultFresnelConfig,
		...fresnelConfig,
	});
	const resolvedAtmosphereConfig = $derived({
		...defaultAtmosphereConfig,
		...atmosphereConfig,
	});

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

	const atmosphereMaterial = new THREE.ShaderMaterial({
		vertexShader: atmosphereVertexShader,
		fragmentShader: atmosphereFragmentShader,
		uniforms: {
			color: { value: new THREE.Color(defaultAtmosphereConfig.color) },
			power: { value: defaultAtmosphereConfig.power },
			coefficient: { value: defaultAtmosphereConfig.coefficient },
			intensity: { value: defaultAtmosphereConfig.intensity },
		},
		side: THREE.BackSide,
		blending: THREE.AdditiveBlending,
		transparent: true,
		depthWrite: false,
		toneMapped: false,
	});

	$effect(() => {
		let cancelled = false;
		void loadLandMask(landTextureUrl).then((mask) => {
			if (!cancelled) {
				landMask = mask;
			}
		});

		return () => {
			cancelled = true;
		};
	});

	$effect(() => {
		material.uniforms.color.value.set(resolvedFresnelConfig.color);
		material.uniforms.rimColor.value.set(resolvedFresnelConfig.rimColor);
		material.uniforms.rimPower.value = resolvedFresnelConfig.rimPower;
		material.uniforms.rimIntensity.value = resolvedFresnelConfig.rimIntensity;
		material.needsUpdate = true;
	});

	let currentAtmosphereScale = $derived(resolvedAtmosphereConfig.scale);

	$effect(() => {
		atmosphereMaterial.uniforms.color.value.set(resolvedAtmosphereConfig.color);
		atmosphereMaterial.uniforms.power.value = resolvedAtmosphereConfig.power;
		atmosphereMaterial.uniforms.coefficient.value =
			resolvedAtmosphereConfig.coefficient;
		atmosphereMaterial.uniforms.intensity.value =
			resolvedAtmosphereConfig.intensity;
		atmosphereMaterial.needsUpdate = true;
	});

	let filteredPositions = $derived.by(() => {
		if (!landMask) {
			return new Float32Array();
		}

		const count = Math.max(1, Math.floor(pointCount));
		const tempPositions: number[] = [];
		const goldenAngle = Math.PI * (3 - Math.sqrt(5));
		const surfaceRadius = radius * 1.001;

		for (let i = 0; i < count; i++) {
			const t = count === 1 ? 0.5 : i / (count - 1);
			const y = 1 - 2 * t;
			const radial = Math.sqrt(Math.max(0, 1 - y * y));
			const theta = goldenAngle * i;
			const x = Math.cos(theta) * radial;
			const z = Math.sin(theta) * radial;

			const pX = x * surfaceRadius;
			const pY = y * surfaceRadius;
			const pZ = z * surfaceRadius;

			if (isPointOnLand(pX, pY, pZ, landMask)) {
				tempPositions.push(pX, pY, pZ);
			}
		}

		return new Float32Array(tempPositions);
	});
	let meshCount = $derived(
		filteredPositions ? filteredPositions.length / 3 : 0,
	);

	$effect(() => {
		if (!focusOn || !$camera || !controls) {
			focusTween?.kill();
			focusTween = null;
			return;
		}

		const [lat, lon] = focusOn;
		const cameraDistance = initialCameraPosition.z;

		const { x, y, z } = lonLatToCartesian(lon, lat, cameraDistance);

		focusTween?.kill();
		focusTween = gsap.to($camera.position, {
			x,
			y,
			z,
			duration: 1.5,
			ease: "power2.inOut",
			onUpdate: () => {
				controls?.update();
			},
			overwrite: true,
		});

		return () => {
			focusTween?.kill();
			focusTween = null;
		};
	});

	function updateMeshMatrices(
		mesh: THREE.InstancedMesh,
		positions: Float32Array,
	) {
		const dummy = new THREE.Object3D();
		const count = positions.length / 3;
		for (let i = 0; i < count; i++) {
			const x = positions[i * 3];
			const y = positions[i * 3 + 1];
			const z = positions[i * 3 + 2];
			dummy.position.set(x, y, z);
			dummy.lookAt(x * 2, y * 2, z * 2);
			dummy.updateMatrix();
			mesh.setMatrixAt(i, dummy.matrix);
		}
		mesh.instanceMatrix.needsUpdate = true;
	}

	function loadLandMask(url: string): Promise<LandMaskData | null> {
		return new Promise((resolve) => {
			const image = new Image();
			image.onload = () => {
				const canvas = document.createElement("canvas");
				canvas.width = image.width;
				canvas.height = image.height;
				const context = canvas.getContext("2d", { willReadFrequently: true });
				if (!context) {
					resolve(null);
					return;
				}

				context.drawImage(image, 0, 0);
				const imageData = context.getImageData(0, 0, image.width, image.height);
				resolve({
					width: image.width,
					height: image.height,
					data: imageData.data,
				});
			};
			image.onerror = (error) => {
				console.warn("GlobeScene: failed to load land mask texture", error);
				resolve(null);
			};
			image.src = url;
		});
	}

	function fract(value: number): number {
		return value - Math.floor(value);
	}

	function pointToMaskUV(
		x: number,
		y: number,
		z: number,
	): { u: number; v: number } {
		const length = Math.sqrt(x * x + y * y + z * z);
		if (length === 0) {
			return { u: 0, v: 0 };
		}

		// Match COBE's globe-space convention before UV mapping:
		// our axes [x,y,z] -> cobe axes [z,y,-x]
		const nx = z / length;
		const ny = y / length;
		const nz = -x / length;

		const gPhi = Math.asin(THREE.MathUtils.clamp(ny, -1, 1));
		const cosPhi = Math.cos(gPhi);

		let gTheta = 0;
		if (Math.abs(cosPhi) > EPSILON) {
			const thetaInput = THREE.MathUtils.clamp(-nx / cosPhi, -1, 1);
			gTheta = Math.acos(thetaInput);
			if (nz < 0) {
				gTheta = -gTheta;
			}
		}

		return {
			u: fract((gTheta * 0.5) / Math.PI),
			v: fract(-(gPhi / Math.PI + 0.5)),
		};
	}

	function sampleLandMask(mask: LandMaskData, u: number, v: number): number {
		const x = Math.min(mask.width - 1, Math.max(0, Math.floor(u * mask.width)));
		const y = Math.min(
			mask.height - 1,
			Math.max(0, Math.floor(v * mask.height)),
		);
		const i = (y * mask.width + x) * 4;
		return mask.data[i] / 255;
	}

	function lonLatToCartesian(lon: number, lat: number, r: number) {
		const lonRad = lon * DEG2RAD;
		const latRad = lat * DEG2RAD;

		const y = r * Math.sin(latRad);
		const rXZ = r * Math.cos(latRad);
		const x = rXZ * Math.sin(lonRad);
		const z = rXZ * Math.cos(lonRad);

		return { x, y, z };
	}

	function isPointOnLand(
		x: number,
		y: number,
		z: number,
		mask: LandMaskData,
	): boolean {
		const { u, v } = pointToMaskUV(x, y, z);
		return sampleLandMask(mask, u, v) >= LAND_MASK_THRESHOLD;
	}
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
		bind:ref={controls}
		enableDamping
		{autoRotate}
		minPolarAngle={lockedPolarAngle ? 1.5 : 0}
		maxPolarAngle={lockedPolarAngle ? 1.4 : Math.PI}
		enableZoom={false}
		oncreate={(c) => {
			c.target.set(0, 0, 0);
			c.update();
		}}
	/>
</T.PerspectiveCamera>

<T.Group bind:ref={globeGroup}>
	<T.Mesh {geometry} {material} />
	<T.Mesh
		{geometry}
		material={atmosphereMaterial}
		scale={currentAtmosphereScale}
	/>

	{#if filteredPositions && meshCount > 0}
		{#key meshCount}
			<T.InstancedMesh
				args={[undefined, undefined, meshCount]}
				oncreate={(mesh) => updateMeshMatrices(mesh, filteredPositions!)}
			>
				<T.CircleGeometry args={[pointSize * 0.5, 6]} />
				<T.MeshBasicMaterial
					color={landPointColor}
					side={THREE.DoubleSide}
					blending={THREE.AdditiveBlending}
					transparent
					depthWrite={false}
					toneMapped={false}
				/>
			</T.InstancedMesh>
		{/key}
	{/if}

	{#each markers as marker, i (marker.label || i)}
		{@const pos = lonLatToCartesian(
			marker.location[1],
			marker.location[0],
			radius,
		)}
		<GlobeMarkerItem
			{marker}
			index={i}
			position={[pos.x, pos.y, pos.z]}
			tooltip={markerTooltip}
		/>
	{/each}
</T.Group>
