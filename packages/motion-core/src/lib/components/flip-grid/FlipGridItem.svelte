<script lang="ts">
  import { cn } from '../../utils/cn';
  
  let { 
    children, 
    class: className = undefined, 
    id, 
    style = undefined, 
    ...props 
  } = $props();
</script>

<div 
  class={cn("flip-grid-item", className)}
  data-flip-id={id}
  {style}
  {...props}
>
  {@render children?.()}
</div>
