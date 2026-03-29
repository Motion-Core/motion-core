<script lang="ts">
	import { T, useTask, useThrelte } from "@threlte/core";
	import { HTML } from "@threlte/extras";
	import * as THREE from "three";
	import type { Snippet } from "svelte";
	import type { GlobeMarker, GlobeMarkerTooltipContext } from "./types";

	interface Props {
		/**
		 * The marker data object containing location, color, size, etc.
		 */
		marker: GlobeMarker;
		/**
		 * The 3D world position of the marker [x, y, z].
		 */
		position: [number, number, number] | { x: number; y: number; z: number };
		/**
		 * Marker index in the markers array.
		 */
		index: number;
		/**
		 * Optional custom tooltip snippet.
		 */
		tooltip?: Snippet<[GlobeMarkerTooltipContext]>;
	}

	let { marker, index, position, tooltip }: Props = $props();

	let tooltipVisibility = $state(1);
	let tooltipBlur = $state(0);

	let group = $state<THREE.Group>();

	const { camera } = useThrelte();
	const markerDirection = new THREE.Vector3();
	const cameraDirection = new THREE.Vector3();
	const worldPosition = new THREE.Vector3();
	const origin = new THREE.Vector3(0, 0, 0);

	const MAX_TOOLTIP_BLUR = 8;
	const VISIBILITY_MIN_DOT = 0.24;
	const VISIBILITY_MAX_DOT = 0.48;

	function cubicBezierAt(
		t: number,
		p0: number,
		p1: number,
		p2: number,
		p3: number,
	): number {
		const u = 1 - t;
		return (
			u * u * u * p0 + 3 * u * u * t * p1 + 3 * u * t * t * p2 + t * t * t * p3
		);
	}

	function cubicBezierDerivativeAt(
		t: number,
		p0: number,
		p1: number,
		p2: number,
		p3: number,
	): number {
		const u = 1 - t;
		return (
			3 * u * u * (p1 - p0) + 6 * u * t * (p2 - p1) + 3 * t * t * (p3 - p2)
		);
	}

	function dynamicEase(value: number): number {
		const clamped = THREE.MathUtils.clamp(value, 0, 1);
		let t = clamped;
		for (let i = 0; i < 5; i++) {
			const x = cubicBezierAt(t, 0, 0.625, 0, 1);
			const dx = cubicBezierDerivativeAt(t, 0, 0.625, 0, 1);
			if (Math.abs(dx) < 1e-6) break;
			t = THREE.MathUtils.clamp(t - (x - clamped) / dx, 0, 1);
		}
		return cubicBezierAt(t, 0, 0.05, 1, 1);
	}

	useTask(() => {
		if (group && $camera) {
			group.getWorldPosition(worldPosition);
			markerDirection.copy(worldPosition).normalize();
			cameraDirection.copy($camera.position).normalize();

			const frontDot = markerDirection.dot(cameraDirection);
			const rawVisibility = THREE.MathUtils.smoothstep(
				frontDot,
				VISIBILITY_MIN_DOT,
				VISIBILITY_MAX_DOT,
			);
			const visibility = dynamicEase(rawVisibility);

			tooltipVisibility = visibility;
			tooltipBlur = (1 - visibility) * MAX_TOOLTIP_BLUR;
		}
	});

	let color = $derived(new THREE.Color(marker.color || "#ffffff"));
	let pointRadius = $derived(Math.max(0.001, marker.size ?? 0.05));
	let tooltipContext = $derived<GlobeMarkerTooltipContext>({
		marker,
		index,
		visibility: tooltipVisibility,
	});
	let markerOpacity = $derived(tooltipVisibility);
	let normalizedPosition = $derived(
		Array.isArray(position)
			? position
			: ([position.x, position.y, position.z] as [number, number, number]),
	);

	$effect(() => {
		if (!group || !normalizedPosition) return;
		group.lookAt(origin);
	});
</script>

<T.Group bind:ref={group} position={normalizedPosition}>
	<T.Mesh renderOrder={10}>
		<T.CircleGeometry args={[pointRadius, 24]} />
		<T.MeshBasicMaterial
			{color}
			side={THREE.DoubleSide}
			transparent
			opacity={markerOpacity}
			depthTest={false}
			depthWrite={false}
			toneMapped={false}
		/>
	</T.Mesh>

	{#if tooltip || marker.label}
		<HTML position={[0, 0, 0]} center>
			<div
				class="pointer-events-none inline-flex -translate-y-6 flex-col items-center transition-[opacity,filter] duration-200 ease-out"
				style:opacity={tooltipVisibility}
				style:filter={`blur(${tooltipBlur}px)`}
			>
				{#if tooltip}
					{@render tooltip(tooltipContext)}
				{:else}
					<div
						class="bg-fixed-dark/80 rounded-xs px-2 py-1 text-xs whitespace-nowrap text-fixed-light backdrop-blur-sm"
					>
						{marker.label}
					</div>
				{/if}
			</div>
		</HTML>
	{/if}
</T.Group>
