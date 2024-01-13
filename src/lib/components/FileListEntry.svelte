<script lang="ts">
	import Svg from './Svg.svelte';
	import type { FileData } from '$lib/types';
	import { FileType } from '$lib/types';
	import { settings, selectedFiles } from '$lib/stores';
	import { getIconPath } from '$lib/utils/icon_resolver';

	export let file: FileData = {
		name: 'undefined',
		type: FileType.FILE,
		size: 0,
		lastModifiedDate: Date.now(),
		createdDate: Date.now(),
		owner: 'undefined',
		permissions: 'undefined'
	};

	export let onSelected: (e: Event, file: FileData, selected: boolean) => void = () => {};

	let src = getIconPath(file.type, $settings.appearance.iconTheme);
	let selected = false;
	let fileDataString: string[] = [];

	$: selected = $selectedFiles.files.includes(file.name);
	$: {
		fileDataString = [];
		fileDataString.push(file.name);
		fileDataString.push(parseDate(file.lastModifiedDate));
		fileDataString.push(parseDate(file.createdDate));
		fileDataString.push(file.owner);
		fileDataString.push(file.permissions);
		fileDataString.push(parseSize(file.size));
	}

	const parseSize = (size: number) => {
		if (size < 1024) {
			return `${size} B`;
		} else if (size < 1024 * 1024) {
			return `${(size / 1024).toFixed(2)} KB`;
		} else if (size < 1024 * 1024 * 1024) {
			return `${(size / 1024 / 1024).toFixed(2)} MB`;
		} else {
			return `${(size / 1024 / 1024 / 1024).toFixed(2)} GB`;
		}
	};

	const parseDate = (timestamp: number) => {
		const date = new Date(timestamp);
		return `${date.toLocaleDateString()} ${date.toLocaleTimeString()}`;
	};

	const clickHandler = (e: Event) => {
		onSelected(e, file, selected);
	};
</script>

<button
	class="file-list-entry {selected ? 'file-list-entry-selected' : ''}"
	style="font-size: {18 * $settings.appearance.zoom}px;"
	on:click={clickHandler}
>
	<div class="file-list-entry-icon">
		<Svg {src} width={40 * $settings.appearance.zoom} height={40 * $settings.appearance.zoom} />
	</div>
	<div class="file-list-entry-data">
		{#each $settings.fileList.fileListHeaders as header, i}
			{#if header.active}
				<div class="file-list-entry-info" style="width: {header.width}em;">{fileDataString[i]}</div>
			{/if}
		{/each}
	</div>
</button>

<style>
	.file-list-entry {
		display: flex;
		flex-direction: row;
		align-items: center;
		border-radius: 0.63em;
		min-height: 3.15em;
		background: transparent;
		color: inherit;
		font: inherit;
		outline: none;
		padding: 0;
		text-align: inherit;
		text-decoration: none;
		border: none;
		margin-bottom: 0.275em;
	}

	.file-list-entry:hover {
		background-color: var(--color-secondary);
	}

	.file-list-entry-selected {
		background-color: var(--color-secondary);
	}

	.file-list-entry-icon {
		margin: 0.315em 0.945em 0 0.63em;
		pointer-events: none;
		user-select: none;
	}

	.file-list-entry-data {
		user-select: none;
		display: flex;
		flex-direction: row;
	}

	.file-list-entry-info {
		margin-right: 0.945em;
		overflow: hidden;
		text-overflow: ellipsis;
		white-space: nowrap;
		font-size: 1.26em;
	}
</style>
