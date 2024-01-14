<script lang="ts">
	import { settings } from '$lib/stores';
	import Svg from '$lib/components/Svg.svelte';
	import { SortType } from '$lib/types';

	export let width = 0;
	export let text = '';
	export let type: SortType = SortType.NAME;
	export let onSort: (test: SortType) => void;
	export let mouseDown: (e: MouseEvent, pxEmConversion: number, type: SortType) => void;
	export let headerWidth = 0;

	let searchIcon = 'none';
	let element: HTMLElement;

	$: {
		if (type === $settings.fileList.sortType) {
			if ($settings.fileList.sortAscending) {
				searchIcon = 'up';
			} else {
				searchIcon = 'down';
			}
		} else {
			searchIcon = 'none';
		}
	}

	const clickHandler = (e: Event) => {
		onSort(type);
	};

	const mouseDownHandler = (e: MouseEvent) => {
		mouseDown(e, 1 / parseFloat(getComputedStyle(element).fontSize), type);
	};
</script>

<div class="file-list-header-entry reset-button" bind:clientWidth={headerWidth}>
	<button class="reset-button" on:click={clickHandler}>
		<div class="file-list-header-info" style="--width: calc({width}em + .1em);" bind:this={element}>{text}</div>
		<div class="file-list-header-sort">
			{#if searchIcon === 'up'}
				<Svg
					src="/svgs/ui/sort_arrow_down.svg"
					width={20 * $settings.appearance.zoom}
					height={20 * $settings.appearance.zoom}
				/>
			{:else if searchIcon === 'down'}
				<Svg
					src="/svgs/ui/sort_arrow_up.svg"
					width={20 * $settings.appearance.zoom}
					height={20 * $settings.appearance.zoom}
				/>
			{/if}
		</div>
	</button>
	<button class="file-list-header-entry-drag reset-button" on:mousedown={mouseDownHandler}></button>
</div>

<style>
	.file-list-header-entry {
		height: 100%;
		border-radius: 0.63em;
	}

	.file-list-header-entry:hover {
		background-color: var(--color-secondary);
	}

	.file-list-header-info {
		min-width: calc(var(--width) - 2.3em);
		width: calc(var(--width) - 2.3em);
		overflow: hidden;
		text-overflow: ellipsis;
		white-space: nowrap;
		font-size: 1.26em;
		user-select: none;
		padding-left: .85em;
	}

	.file-list-header-sort {
		display: flex;
		flex-direction: row;
		align-items: center;
		justify-content: center;
		width: 2.52em;
		height: 2.52em;
		user-select: none;
		pointer-events: none;
	}

	.file-list-header-entry-drag {
		width: 0.5em;
		height: 100%;
		cursor: col-resize;
		border-right: 1px solid var(--color-secondary);
	}
</style>
