<script lang="ts">
	import { T, useThrelte } from "@threlte/core";
	import { OrbitControls, HTML, interactivity } from "@threlte/extras";
	import * as THREE from "three";
	import type { OrbitControls as OrbitControlsType } from "three/examples/jsm/controls/OrbitControls.js";
	import gsap from "gsap";
	import landGeoJsonRaw from "../../assets/ne_110m_land.geojson?raw";
	import type { GlobeMarker } from "./types";
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
		 * @default 3
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
		 * @default "#ffffff"
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
		 * @default 1.0
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
		 * @default "#FF6900"
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
		 * Coordinates [lat, lon] to focus on.
		 */
		focusOn?: [number, number] | null;
	}

	type GeoJSONPolygon = {
		type: "Polygon";
		coordinates: number[][][];
	};

	type GeoJSONMultiPolygon = {
		type: "MultiPolygon";
		coordinates: number[][][][];
	};

	type GeoJSONGeometry = GeoJSONPolygon | GeoJSONMultiPolygon;

	type GeoJSONFeature = {
		type: "Feature";
		geometry: GeoJSONGeometry | null;
	};

	type GeoJSONFeatureCollection = {
		type: "FeatureCollection";
		features: GeoJSONFeature[];
	};

	type SphericalPoint = [lon: number, lat: number];

	interface BoundingBox {
		minLon: number;
		maxLon: number;
		minLat: number;
		maxLat: number;
	}

	interface ParsedPolygon {
		rings: SphericalPoint[][];
		bbox: BoundingBox;
	}

	const DEG2RAD = Math.PI / 180;
	const EPSILON = 1e-9;

	const landPolygons = parseLandPolygons(landGeoJsonRaw);

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
		focusOn = null,
	}: Props = $props();

	const initialCameraPosition = { x: 0, y: 0, z: 8 };
	let globeGroup = $state<THREE.Group>();
	let controls = $state<OrbitControlsType>();

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

	let currentAtmosphereScale = $state(defaultAtmosphereConfig.scale);

	$effect(() => {
		const config = {
			...defaultAtmosphereConfig,
			...atmosphereConfig,
		};

		atmosphereMaterial.uniforms.color.value.set(config.color);
		atmosphereMaterial.uniforms.power.value = config.power;
		atmosphereMaterial.uniforms.coefficient.value = config.coefficient;
		atmosphereMaterial.uniforms.intensity.value = config.intensity;
		currentAtmosphereScale = config.scale;
		atmosphereMaterial.needsUpdate = true;
	});

	let filteredPositions = $state<Float32Array | null>(null);
	let meshCount = $derived(
		filteredPositions ? filteredPositions.length / 3 : 0,
	);

	$effect(() => {
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

			const { lon, lat } = cartesianToLonLat(pX, pY, pZ);

			if (isPointOnLand(lon, lat)) {
				tempPositions.push(pX, pY, pZ);
			}
		}

		filteredPositions = new Float32Array(tempPositions);
	});

	$effect(() => {
		if (focusOn && $camera && controls) {
			const [lat, lon] = focusOn;
			const cameraDistance = initialCameraPosition.z;

			const { x, y, z } = lonLatToCartesian(lon, lat, cameraDistance);

			gsap.to($camera.position, {
				x,
				y,
				z,
				duration: 1.5,
				ease: "power2.inOut",
				onUpdate: () => {
					controls?.update();
				},
			});
		}
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

	function parseLandPolygons(raw: string): ParsedPolygon[] {
		try {
			const collection = JSON.parse(raw) as GeoJSONFeatureCollection;
			return extractPolygons(collection);
		} catch (error) {
			console.warn("GlobeScene: failed to parse land GeoJSON", error);
			return [];
		}
	}

	function extractPolygons(
		collection: GeoJSONFeatureCollection,
	): ParsedPolygon[] {
		const polygons: ParsedPolygon[] = [];
		for (const feature of collection.features ?? []) {
			const geometry = feature.geometry;
			if (!geometry) continue;
			if (geometry.type === "Polygon") {
				const polygon = convertPolygon(geometry.coordinates);
				if (polygon) {
					polygons.push(polygon);
				}
				continue;
			}
			if (geometry.type === "MultiPolygon") {
				for (const coords of geometry.coordinates) {
					const polygon = convertPolygon(coords);
					if (polygon) {
						polygons.push(polygon);
					}
				}
			}
		}
		return polygons;
	}

	function convertPolygon(rings: number[][][]): ParsedPolygon | null {
		const converted = rings
			.map((ring) =>
				ring.map(
					([lon, lat]) => [lon * DEG2RAD, lat * DEG2RAD] as SphericalPoint,
				),
			)
			.filter((ring) => ring.length >= 3);

		if (!converted.length) {
			return null;
		}

		return {
			rings: converted,
			bbox: computeBoundingBox(converted[0]),
		};
	}

	function computeBoundingBox(ring: SphericalPoint[]): BoundingBox {
		const bbox: BoundingBox = {
			minLon: Infinity,
			maxLon: -Infinity,
			minLat: Infinity,
			maxLat: -Infinity,
		};

		for (const [lon, lat] of ring) {
			bbox.minLon = Math.min(bbox.minLon, lon);
			bbox.maxLon = Math.max(bbox.maxLon, lon);
			bbox.minLat = Math.min(bbox.minLat, lat);
			bbox.maxLat = Math.max(bbox.maxLat, lat);
		}

		return bbox;
	}

	function cartesianToLonLat(
		x: number,
		y: number,
		z: number,
	): { lon: number; lat: number } {
		const radius = Math.sqrt(x * x + y * y + z * z);
		if (radius === 0) {
			return { lon: 0, lat: 0 };
		}
		const lat = Math.asin(Math.min(1, Math.max(-1, y / radius)));
		const lon = Math.atan2(x, z);
		return { lon, lat };
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

	function isPointOnLand(lon: number, lat: number): boolean {
		for (const polygon of landPolygons) {
			if (!isWithinBounds(lon, lat, polygon.bbox)) continue;
			if (isPointInsidePolygon(lon, lat, polygon.rings)) {
				return true;
			}
		}
		return false;
	}

	function isWithinBounds(
		lon: number,
		lat: number,
		bbox: BoundingBox,
	): boolean {
		return (
			lon >= bbox.minLon - EPSILON &&
			lon <= bbox.maxLon + EPSILON &&
			lat >= bbox.minLat - EPSILON &&
			lat <= bbox.maxLat + EPSILON
		);
	}

	function isPointInsidePolygon(
		lon: number,
		lat: number,
		rings: SphericalPoint[][],
	): boolean {
		if (!rings.length) return false;
		if (!isPointInRing(lon, lat, rings[0])) return false;
		for (let i = 1; i < rings.length; i++) {
			if (isPointInRing(lon, lat, rings[i])) {
				return false;
			}
		}
		return true;
	}

	function isPointInRing(
		lon: number,
		lat: number,
		ring: SphericalPoint[],
	): boolean {
		let inside = false;
		for (let i = 0, j = ring.length - 1; i < ring.length; j = i++) {
			const xi = ring[i][0];
			const yi = ring[i][1];
			const xj = ring[j][0];
			const yj = ring[j][1];
			const denom = yj - yi;
			if (Math.abs(denom) < EPSILON) {
				continue;
			}
			const intersects =
				yi > lat !== yj > lat && lon < ((xj - xi) * (lat - yi)) / denom + xi;
			if (intersects) inside = !inside;
		}
		return inside;
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
					toneMapped={false}
				/>
			</T.InstancedMesh>
		{/key}
	{/if}

	{#each markers as marker, i (marker.label || i)}
		{@const pos = lonLatToCartesian(
			marker.location[1],
			marker.location[0],
			radius * 1.002,
		)}
		<GlobeMarkerItem {marker} {radius} position={[pos.x, pos.y, pos.z]} />
	{/each}
</T.Group>
