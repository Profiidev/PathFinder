<script lang="ts">
	import Svg from '$lib/components/Svg.svelte';
	import { settings } from '$lib/stores';
	import type { SvgData } from '$lib/types';

	export let svgData: SvgData = {
		path: '',
		colors: []
	};
	export let text: string;
</script>

<div class="sidebar-entry" style="width: calc({$settings.sidebar.width}em - 1.5em);">
	<div class="sidebar-entry-icon">
		{#if svgData.path !== ''}
			<Svg svgData={{ data: svgData, width: 20, height: 20 }} />
		{:else}
			<slot />
		{/if}
	</div>
	<span>{text}</span>
</div>

<style>
	.sidebar-entry {
		display: flex;
		flex-direction: row;
		align-items: center;
		padding: 0 0.3em;
		height: 1.4em;
		cursor: pointer;
		user-select: none;
		border-radius: 0.25em;
		margin-top: 0.2em;
	}

	.sidebar-entry:hover {
		background-color: var(--color-secondary);
	}

	.sidebar-entry-icon {
		display: flex;
		flex-direction: row;
		align-items: center;
		justify-content: center;
		margin-right: 0.5em;
		pointer-events: none;
	}

	.sidebar-entry span {
		padding-top: 0.1em;
		text-overflow: ellipsis;
		overflow: hidden;
		white-space: nowrap;
		font-size: 0.75em;
	}
</style>
