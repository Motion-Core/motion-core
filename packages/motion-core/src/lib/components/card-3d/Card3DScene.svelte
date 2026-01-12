<script lang="ts">
	import { T, useTask } from "@threlte/core";
	import { useTexture } from "@threlte/extras";
	import * as THREE from "three";

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

	const initialCameraPosition = { x: 0, y: 0, z: 5 };

	let smoothedRotation = $state({ x: 0, y: 0 });

	const lerpFactor = 0.1;

	useTask(() => {
		const targetRotationY = -headPosition.x * 0.5;
		const targetRotationX = headPosition.y * 0.4;

		smoothedRotation.x += (targetRotationX - smoothedRotation.x) * lerpFactor;
		smoothedRotation.y += (targetRotationY - smoothedRotation.y) * lerpFactor;
	});

	function createRoundedRectShape(
		w: number,
		h: number,
		r: number,
	): THREE.Shape {
		const shape = new THREE.Shape();

		const maxRadius = Math.min(w, h) / 2;
		const clampedRadius = Math.min(r, maxRadius);

		const halfW = w / 2;
		const halfH = h / 2;

		shape.moveTo(-halfW + clampedRadius, -halfH);
		shape.lineTo(halfW - clampedRadius, -halfH);
		shape.quadraticCurveTo(halfW, -halfH, halfW, -halfH + clampedRadius);
		shape.lineTo(halfW, halfH - clampedRadius);
		shape.quadraticCurveTo(halfW, halfH, halfW - clampedRadius, halfH);
		shape.lineTo(-halfW + clampedRadius, halfH);
		shape.quadraticCurveTo(-halfW, halfH, -halfW, halfH - clampedRadius);
		shape.lineTo(-halfW, -halfH + clampedRadius);
		shape.quadraticCurveTo(-halfW, -halfH, -halfW + clampedRadius, -halfH);

		return shape;
	}

	let geometry = $derived.by(() => {
		const shape = createRoundedRectShape(width, height, radius);

		const extrudeSettings: THREE.ExtrudeGeometryOptions = {
			depth: depth,
			bevelEnabled: false,
			curveSegments: 16,
		};

		const geo = new THREE.ExtrudeGeometry(shape, extrudeSettings);

		// Center the geometry on Z axis
		geo.translate(0, 0, -depth / 2);

		// UV mapping will be applied after texture loads (to get image aspect ratio)
		return geo;
	});

	const texture = $derived(
		useTexture(image, {
			transform: (tex) => {
				tex.colorSpace = THREE.SRGBColorSpace;
				return tex;
			},
		}),
	);

	// Apply object-cover UV mapping when texture is loaded
	$effect(() => {
		if (!$texture || !geometry) return;

		const imageAspect = $texture.image.width / $texture.image.height;
		const cardAspect = width / height;

		const uvAttribute = geometry.attributes.uv;
		const positionAttribute = geometry.attributes.position;
		const normalAttribute = geometry.attributes.normal;

		// Calculate scale and offset for "cover" behavior
		let scaleU = 1;
		let scaleV = 1;
		let offsetU = 0;
		let offsetV = 0;

		if (cardAspect > imageAspect) {
			// Card is wider than image - crop top/bottom
			scaleV = imageAspect / cardAspect;
			offsetV = (1 - scaleV) / 2;
		} else {
			// Card is taller than image - crop left/right
			scaleU = cardAspect / imageAspect;
			offsetU = (1 - scaleU) / 2;
		}

		for (let i = 0; i < uvAttribute.count; i++) {
			const nz = normalAttribute.getZ(i);

			// Only remap UVs for front face (nz > 0.9) and back face (nz < -0.9)
			if (Math.abs(nz) > 0.9) {
				const x = positionAttribute.getX(i);
				const y = positionAttribute.getY(i);

				// Map position to 0-1 range
				let u = (x + width / 2) / width;
				let v = (y + height / 2) / height;

				// Apply cover scaling and offset
				u = u * scaleU + offsetU;
				v = v * scaleV + offsetV;

				uvAttribute.setXY(i, u, v);
			}
		}

		uvAttribute.needsUpdate = true;
	});

	let material = $state<THREE.MeshBasicMaterial | null>(null);

	$effect(() => {
		if ($texture) {
			material = new THREE.MeshBasicMaterial({
				map: $texture,
			});
		}
	});
</script>

<T.PerspectiveCamera
	makeDefault
	position={[
		initialCameraPosition.x,
		initialCameraPosition.y,
		initialCameraPosition.z,
	]}
	fov={50}
/>

<T.Group rotation.x={smoothedRotation.x} rotation.y={smoothedRotation.y}>
	{#if material}
		<T.Mesh {geometry} {material} />
	{/if}
</T.Group>
