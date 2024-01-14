<script lang="ts">
  import { settings } from '$lib/stores';
  import Svg from '$lib/components/Svg.svelte';

  let svgStyle = 'display: flex; flex-direction: row; align-items: center; justify-content: center;';
</script>

<div class="topbar">
  <div class="topbar-left">
    <div class="topbar-button">
      <Svg src="/svgs/ui/arrow_back.svg" width={25} height={25} style={svgStyle}/>
    </div>
    <div class="topbar-button">
      <Svg src="/svgs/ui/arrow_forward.svg" width={25} height={25} style={svgStyle}/>
    </div>
    <div class="topbar-button">
      <Svg src="/svgs/ui/arrow_up.svg" width={25} height={25} style={svgStyle}/>
    </div>
    <div class="topbar-button">
      <Svg src="/svgs/ui/arrow_repeat.svg" width={18.75} height={18.75} style={svgStyle}/>
    </div>
    <input type="text" placeholder="Path" class="topbar-path" />
  </div>
</div>

<style>
  .topbar {
    display: flex;
    flex-direction: row;
    align-items: center;
    justify-content: space-between;
    background-color: var(--color-primary);
    color: var(--color-text);
    padding: 0 .2em;
    height: 100%;
    width: 100%;
    font-size: 1em;
  }

  .topbar-left {
    display: flex;
    flex-direction: row;
    align-items: center;
    justify-content: flex-start;
  }

  .topbar-button {
    display: flex;
    flex-direction: row;
    align-items: center;
    justify-content: center;
    background-color: var(--color-primary);
    color: var(--color-text);
    padding: 0.5em;
    margin: 0 0.25em;
    border-radius: 0.25em;
    cursor: pointer;
    width: 20px;
    height: 20px;
  }

  .topbar-button:hover {
    background-color: var(--color-secondary);
  }

  .topbar-path {
    background-color: var(--color-tertiary);
    color: var(--color-text);
    border: none;
    outline: none;
    font-size: 1em;
    width: 100%;
    height: 100%;
    margin: 0 0.5em;
    border-radius: 0.25em;
    height: 35px;
    width: 500px;
    padding: 0 0.5em;
  }

  .topbar-path:focus {
    background-color: var(--color-primary);
    border: 1px solid var(--color-secondary);
    border-bottom: 1px solid var(--color-accent);
  }
</style>