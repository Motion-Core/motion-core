<script lang="ts">
	import { T, useTask } from "@threlte/core";
	import { HTML } from "@threlte/extras";
	import * as THREE from "three";
	import type { GlobeMarker } from "./types";

	interface Props {
		/**
		 * The marker data object containing location, color, size, etc.
		 */
		marker: GlobeMarker;
		/**
		 * The radius of the globe sphere. Used to calculate surface projection.
		 */
		radius: number;
		/**
		 * The 3D world position of the marker [x, y, z].
		 */
		position: [number, number, number] | { x: number; y: number; z: number };
		/**
		 * Callback when the pointer enters the marker area.
		 */
		onpointerenter?: () => void;
		/**
		 * Callback when the pointer leaves the marker area.
		 */
		onpointerleave?: () => void;
	}

	let { marker, radius, position, onpointerenter, onpointerleave }: Props =
		$props();

	let isHovered = $state(false);

	function handlePointerEnter() {
		isHovered = true;
		onpointerenter?.();
	}

	function handlePointerLeave() {
		isHovered = false;
		onpointerleave?.();
	}

	const vertexShader = `
        uniform float uRadius;
        varying vec2 vUv;

        void main() {
            vUv = uv;

            vec4 worldPos = modelMatrix * vec4(position, 1.0);

            vec3 projectedPos = normalize(worldPos.xyz) * (uRadius * 1.002);

            gl_Position = projectionMatrix * viewMatrix * vec4(projectedPos, 1.0);
        }
    `;

	const fragmentShader = `
        uniform vec3 uColor;
        uniform float uTime;
        varying vec2 vUv;

        void main() {
            float dist = distance(vUv, vec2(0.5));

            // Create circular mask
            if (dist > 0.5) discard;

            float coreRadius = 0.125;
            float core = 1.0 - smoothstep(coreRadius - 0.01, coreRadius + 0.01, dist);

            float frequency = 15.0;
            float speed = 2.5;
            float phase = dist * frequency - uTime * speed;

            float wave = sin(phase);

            float ring = smoothstep(0.5, 1.0, wave);

            float fade = pow(1.0 - smoothstep(coreRadius, 0.5, dist), 2.0);

            float alpha = max(core, ring * fade * 0.8);

            vec3 finalColor = uColor;

            gl_FragColor = vec4(finalColor, alpha);
            #include <colorspace_fragment>
        }
    `;

	const uniforms = {
		uTime: { value: 0 },
		uColor: { value: new THREE.Color("#ffffff") },
		uRadius: { value: 1 },
	};

	useTask((delta) => {
		uniforms.uTime.value += delta;
	});

	$effect(() => {
		uniforms.uColor.value.set(marker.color || "#ffffff");
		uniforms.uRadius.value = radius;
	});

	let scale = $derived((marker.size || 0.05) * 4);
	let normalizedPosition = $derived(
		Array.isArray(position)
			? position
			: ([position.x, position.y, position.z] as [number, number, number]),
	);

	let mesh = $state<THREE.Mesh>();

	$effect(() => {
		if (mesh) {
			mesh.lookAt(new THREE.Vector3(0, 0, 0));
		}
	});
</script>

<T.Mesh
	bind:ref={mesh}
	onpointerenter={handlePointerEnter}
	onpointerleave={handlePointerLeave}
	position={normalizedPosition}
	scale={[scale, scale, scale]}
>
	<T.PlaneGeometry args={[1, 1, 32, 32]} />
	<T.ShaderMaterial
		{vertexShader}
		{fragmentShader}
		{uniforms}
		transparent
		depthWrite={false}
		toneMapped={false}
		side={THREE.DoubleSide}
	/>

	{#if marker.label}
		<HTML position={[0, 0, 0]} center pointerEvents="none">
			<div
				class="pointer-events-none absolute -top-4 left-1/2 -translate-x-1/2 rounded bg-neutral-900/80 px-2 py-1 text-xs whitespace-nowrap text-white backdrop-blur-sm transition-opacity duration-200"
				class:opacity-100={isHovered}
				class:opacity-0={!isHovered}
			>
				{marker.label}
			</div>
		</HTML>
	{/if}
</T.Mesh>
