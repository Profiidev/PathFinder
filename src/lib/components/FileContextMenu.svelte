<script lang="ts">
	import ContextMenu from "./ContextMenu.svelte";
	import Svg from "./Svg.svelte";

  export let visible = false;
  export let fileContextX = 0;
  export let fileContextY = 0;
  export let fileContextData = {};

  const clickHandler = () => {
    visible = false;
  }
</script>

<svelte:window on:click={clickHandler} on:contextmenu={clickHandler} />

<ContextMenu width={7} height={11.2} bind:visible={visible} bind:x={fileContextX} bind:y={fileContextY}>
  <div class="container">
    <button class="menu-button reset-button">
      <div class="placeholder" style="width: 15px;"></div>
      <span class="open">Open</span>
    </button>
    <button class="menu-button reset-button">
      <Svg svgData={ { data: { path: "/svgs/ui/cut.svg", colors: [] }, width: 15, height:15 } }/>
      <span class="cut">Cut</span>
    </button>
    <button class="menu-button reset-button">
      <Svg svgData={ { data: { path: "/svgs/ui/copy.svg", colors: [] }, width: 15, height:15 } }/>
      <span class="copy">Copy</span>
    </button>
    <button class="menu-button reset-button">
      <Svg svgData={ { data: { path: "/svgs/ui/paste.svg", colors: [] }, width: 15, height:15 } }/>
      <span class="paste">Paste</span>
    </button>
    <button class="menu-button reset-button">
      <Svg svgData={ { data: { path: "/svgs/ui/rename.svg", colors: [] }, width: 15, height:15 } }/>
      <span class="rename">Rename</span>
    </button>
    <button class="menu-button reset-button">
      <Svg svgData={ { data: { path: "/svgs/ui/delete.svg", colors: [] }, width: 15, height:15 } }/>
      <span class="delete">Delete</span>
    </button>
    <button class="menu-button reset-button">
      <Svg svgData={ { data: { path: "/svgs/ui/properties.svg", colors: [] }, width: 15, height:15 } }/>
      <span class="properties">Properties</span>
    </button>
  </div>  
</ContextMenu>

<style>
  .container {
    width: 100%;
    height: 100%;
  }

  .menu-button {
    color: var(--color-text);
    display: flex;
    align-items: center;
    width: 100%;
    height: 1.5em;
    border-radius: .25em;
  }

  .menu-button:hover {
    background-color: var(--color-primary);
  }

  .menu-button span {
    margin-left: .5em;
    font-size: .75em;
  }
</style>