<script lang="ts">
	import ContextMenu from './ContextMenu.svelte';
	import { settings, windowSettings } from '$lib/stores';
	import Svg from './Svg.svelte';
	import { getPrimaryColor } from '$lib/utils/icon_resolver';

	export let x = 0;
	export let y = 0;
	export let visible = false;
</script>

<ContextMenu {x} {y} bind:visible={visible} width={14} height={20}>
	<div class="headers">
		{#each $settings.fileList.fileListHeaders as header}
      {#if header.name !== 'Name'}
        <button class="header-button reset-button" on:click={() => {header.active = !header.active}}>
          <div class="is-active">
            <Svg
              svgData={{
                data: {
                  path: header.active ? '/svgs/ui/check.svg' : '',
                  colors: [{ key: 'FFFFFF', color: getPrimaryColor($settings.appearance.iconTheme) }]
                },
                width: 35 * $windowSettings.zoom,
                height: 35 * $windowSettings.zoom
              }}
            />
          </div>
          {header.name}
        </button>
      {/if}
		{/each}
	</div>
</ContextMenu>

<style>
	.headers {
		display: flex;
		flex-direction: column;
		width: 100%;
		height: 100%;
		color: var(--color-text);
	}

	.header-button {
		display: flex;
		align-items: center;
		justify-content: flex-start;
		flex-direction: row;
		width: 100%;
		height: 4em;
		padding: 0.5em;
		font-size: 1.25em;
		cursor: pointer;
    border-radius: .5em;
	}

  .header-button:hover {
    background-color: var(--color-secondary);
  }

	.is-active {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 1.5em;
		height: 1.5em;
		margin-right: 0.5em;
	}
</style>
