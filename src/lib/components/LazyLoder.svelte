<script lang="ts">
  import { onDestroy, onMount } from "svelte";

  export let height = 0;
  export let width = 0;

  let loaded = false;
  let root: HTMLDivElement;

  let observer = new IntersectionObserver((entries) => {
    entries.forEach((entry) => {
      if (entry.isIntersecting) {
        loaded = true;
      } else {
        loaded = false;
      }
    });
  }, {threshold: 0.01});

  onMount(() => {
    observer.observe(root);
  });

  onDestroy(() => {
    observer.disconnect();
  });
</script>

<div bind:this={root} style="height: {height}px; width: {width}px">
  {#if loaded}
    <div>
      <slot />
    </div>
  {:else}
  <div style="height: {height}px; width: {width}px; background-color: var(--color-background);"></div>
  {/if}
</div>