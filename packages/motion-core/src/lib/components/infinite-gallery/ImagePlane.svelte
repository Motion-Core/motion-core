<script lang="ts">
	interface Props {
		/**
		 * Legacy compatibility prop (unused in OGL migration).
		 */
		texture?: unknown;
		/**
		 * Legacy compatibility prop (unused in OGL migration).
		 */
		position?: [number, number, number];
		/**
		 * Legacy compatibility prop (unused in OGL migration).
		 */
		scale?: [number, number, number];
		/**
		 * Legacy compatibility prop (unused in OGL migration).
		 */
		material?: unknown;
	}

	let {
		texture: _texture,
		position: _position = [0, 0, 0],
		scale: _scale = [1, 1, 1],
		material: _material,
	}: Props = $props();
</script>
