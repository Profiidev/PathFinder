<script lang="ts">
	import Svg from './Svg.svelte';
	import type { FileData, FileListHeader } from '$lib/types';
	import { FileType } from '$lib/types';
	import { settings, selectedFiles, windowSettings } from '$lib/stores';
	import { getIconData } from '$lib/utils/icon_resolver';

	export let file: FileData = {
		name: 'undefined',
		type: FileType.FILE,
		path: 'undefined',
		size: 0,
		lastModifiedDate: Date.now(),
		createdDate: Date.now(),
		permissions: 'undefined',
		hidden: false
	};
	export let width = 100;
	export let headers: FileListHeader[] = [];

	export let onSelected: (e: Event, file: FileData, selected: boolean) => void = () => {};

	let svgData = getIconData(file.type, $settings.appearance.iconTheme);
	let selected = false;
	let fileDataString: string[] = [];

	$: selected = $selectedFiles.files.includes(file.path);
	$: {
		fileDataString = [];
		fileDataString.push(parseFileName(file.name));
		fileDataString.push(parseDate(file.lastModifiedDate));
		fileDataString.push(parseDate(file.createdDate));
		fileDataString.push(file.permissions ? 'read' : 'write');
		fileDataString.push(file.type);
		fileDataString.push(parseSize(file.size));
	}
	$: file.type,
		svgData = getIconData(file.type, $settings.appearance.iconTheme)

	const parseFileName = (name: string) => {
		let dotCount = name.split('.').length - 1;
		if (
			((name.charAt(0) === '.' && dotCount >= 2) || (name.charAt(0) !== '.' && dotCount >= 1)) &&
			file.type === FileType.FILE && !$settings.fileList.showFileExtensions
		) {
			return name.substring(0, name.lastIndexOf('.'));
		} else {
			return name;
		}
	};

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
		const date = new Date(timestamp * 1000);
		return `${date.toLocaleDateString()} ${date.toLocaleTimeString()}`;
	};

	const clickHandler = (e: Event) => {
		e.stopPropagation();
		onSelected(e, file, selected);
	};
</script>

<button
	class="file-list-entry reset-button {selected ? 'file-list-entry-selected' : ''}"
	style="font-size: {18 * $windowSettings.zoom}px; width: {width}px;"
	on:click={clickHandler}
>
	<div class="file-list-entry-icon" style={file.hidden ? 'opacity: 0.3;' : ''}>
		<Svg
			svgData={{
				data: svgData,
				width: 40 * $windowSettings.zoom,
				height: 40 * $windowSettings.zoom
			}}
		/>
	</div>
	<div class="file-list-entry-data">
		{#each headers as header, i}
			{#if header.active}
				<div
					class="file-list-entry-info {header.align_text_right
						? 'file-list-entry-info-align-right'
						: ''}"
					style="width: {header.width}em;"
				>
					{fileDataString[i]}
				</div>
			{/if}
		{/each}
	</div>
</button>

<style>
	.file-list-entry {
		border-radius: 0.63em;
		min-height: 3.15em;
		margin-bottom: 0.275em;
	}

	.file-list-entry:hover {
		background-color: var(--color-secondary);
	}

	.file-list-entry-selected {
		background-color: var(--color-secondary);
	}

	.file-list-entry-icon {
		margin: 0 0.945em 0 0.63em;
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

	.file-list-entry-info-align-right {
		text-align: right;
	}
</style>
