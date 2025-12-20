<script lang="ts">
import { cn } from "$lib/utils/cn";

type ComponentProps = Record<string, unknown> & { class?: string };
let { class: className = "", ...restProps }: ComponentProps = $props();
</script>

<hr
	{...restProps}
	class={cn("my-12 border-t border-border", className)}
/>
