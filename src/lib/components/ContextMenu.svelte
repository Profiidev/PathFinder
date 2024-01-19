<script lang="ts">
  export let x = 0;
  export let y = 0;
  export let visible = false;
  export let width = 0;
  export let height = 0;
  
  const clickHandler = () => {
    visible = false;
  }
</script>

<svelte:window on:click={clickHandler} on:contextmenu={clickHandler} />

{#if visible}
  <button class="context-menu reset-button" style="width: {width}em; height: {height}em; left: {x}px; top: {y}px;" on:click|stopPropagation>
    <slot />
  </button>
{/if}

<style>
  .context-menu {
    position: absolute;
    z-index: 50;
    background-color: var(--color-primary-dark);
    border: 1px solid var(--color-secondary);
    border-radius: 4px;
    box-shadow: 0 0 4px 0 rgba(0, 0, 0, 0.5);
    padding: 4px;
  }
</style>