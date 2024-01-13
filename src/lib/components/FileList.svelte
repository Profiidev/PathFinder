<script lang="ts">
	import type { FileData } from '$lib/types';
	import { SortType, FileType } from '$lib/types';
	import FileListEntry from './FileListEntry.svelte';
	import FileListHeaderEntry from './FileListHeaderEntry.svelte';
	import { settings, pressedKeys, selectedFiles } from '$lib/stores';
	import { fileListHeaderMinWidth, zoomStep, maxZoom, minZoom } from '$lib/utils/constants';

	export let files: FileData[] = [];

	$: files = files.sort((a, b) => {
		let result = $settings.fileList.sortAscending ? 1 : -1;
		switch ($settings.fileList.sortType) {
			case SortType.NAME:
				result *= a.name.localeCompare(b.name, undefined, { numeric: true, sensitivity: 'base' });
				break;
			case SortType.SIZE:
				result *= a.size - b.size;
				break;
			case SortType.MODIFIED_DATE:
				result *= a.lastModifiedDate - b.lastModifiedDate;
				break;
			case SortType.CREATED_DATE:
				result *= a.createdDate - b.createdDate;
				break;
			case SortType.OWNER:
				result *= a.owner.localeCompare(b.owner, undefined, { numeric: true, sensitivity: 'base' });
				break;
			case SortType.PERMISSIONS:
				result *= a.permissions.localeCompare(b.permissions, undefined, {
					numeric: true,
					sensitivity: 'base'
				});
				break;
			default:
				result *= 0;
				break;
		}
		if (result === 0) {
			if (a.type === b.type) {
				result = a.name.localeCompare(b.name, undefined, { numeric: true, sensitivity: 'base' });
			} else {
				result = a.type === FileType.DIRECTORY ? -1 : 1;
			}
		}
		return result;
	});

	const onSort = (type: SortType) => {
		if ($settings.fileList.sortType === type) {
			$settings.fileList.sortAscending = !$settings.fileList.sortAscending;
		} else {
			$settings.fileList.sortType = type;
			$settings.fileList.sortAscending = true;
		}
	};

	let lastX = 0;
	let isResizing = false;
	let pxEmConversionFactor = 0;
	let currentType: SortType;

	const mouseDown = (e: MouseEvent, pxEmConversion: number, type: SortType) => {
		if (isResizing) return;
		lastX = e.clientX;
		isResizing = true;
		pxEmConversionFactor = pxEmConversion;
		currentType = type;
	};

	const mouseUpHandler = (e: MouseEvent) => {
		isResizing = false;
	};

	const mouseMoveHandler = (e: MouseEvent) => {
		if (!isResizing) return;

		const delta = e.clientX - lastX;
		lastX = e.clientX;
		let toAdd = Math.round(delta * pxEmConversionFactor * 100) / 100;
    
		let headers = $settings.fileList.fileListHeaders;
    headers.forEach((header) => {
      if (header.sortType === currentType) {
        if (header.width + toAdd < fileListHeaderMinWidth) {
          toAdd = fileListHeaderMinWidth - header.width;
        }
        header.width += toAdd;
      }
    });
    $settings.fileList.fileListHeaders = headers;
	};

	const onSelected = (e: Event, file: FileData, selected: boolean) => {
		selectedFiles.update((data) => {
			if ($pressedKeys.includes('control') && $pressedKeys.includes('shift')) {
				if (data.lastSelectedIndex !== -1) {
					const start = Math.min(data.lastSelectedIndex, files.indexOf(file));
					const end = Math.max(data.lastSelectedIndex, files.indexOf(file));
					data.files = [
						...data.files,
						...files
							.slice(start, end + 1)
							.map((file) => file.name)
							.filter((f) => !data.files.includes(f))
					];
				}
			} else if ($pressedKeys.includes('control')) {
				if (selected) {
					data.files = data.files.filter((f) => f !== file.name);
				} else {
					data.files = [...data.files, file.name];
				}
				data.lastSelectedIndex = files.indexOf(file);
			} else if ($pressedKeys.includes('shift')) {
				if (data.lastSelectedIndex === -1) {
					data.files = files.slice(0, files.indexOf(file) + 1).map((file) => file.name);
				} else {
					const start = Math.min(data.lastSelectedIndex, files.indexOf(file));
					const end = Math.max(data.lastSelectedIndex, files.indexOf(file));
					data.files = files.slice(start, end + 1).map((file) => file.name);
				}
			} else {
				if (selected) {
					if (data.files.length === 1) {
						//TODO change name
					} else {
						data.files = [file.name];
					}
				} else {
					data.files = [file.name];
				}
				data.lastSelectedIndex = files.indexOf(file);
			}
			return data;
		});
	};

	const scrollHandler = (e: Event) => {
		if(!$pressedKeys.includes('control')) return;

		const delta = (e as WheelEvent).deltaY * -zoomStep / 100;
		$settings.appearance.zoom = Math.max(minZoom, Math.min(maxZoom, $settings.appearance.zoom + delta));
	};
</script>

<svelte:body on:mouseup={mouseUpHandler} on:mousemove={mouseMoveHandler} />

<div class="file-list" style="font-size: {18 * $settings.appearance.zoom}px;" on:wheel={scrollHandler}>
	<div class="file-list-header">
		<div class="file-list-header-left" style="min-width: 3.75em;"></div>
		{#each $settings.fileList.fileListHeaders as header}
			{#if header.active}
				<FileListHeaderEntry
					width={header.width}
					type={header.sortType}
					text={header.name}
					{onSort}
					{mouseDown}
				/>
			{/if}
		{/each}
	</div>
	<div class="file-list-items">
		{#each files as file}
			<FileListEntry {file} {onSelected} />
		{/each}
	</div>
</div>

<style>
	.file-list {
		width: 100%;
		margin: 0;
		padding: 0;
		display: flex;
		flex-direction: column;
		color: var(--color-text);
		height: 100%;
	}
	.file-list-header {
		display: flex;
		flex-direction: row;
		align-items: center;
		height: 3.15em;
		padding: 0;
		flex-shrink: 0;
		background-color: var(--color-primary);
		border-bottom: 1px solid var(--color-secondary);
		resize: horizontal;
	}
	.file-list-items {
		display: flex;
		flex-direction: column;
		overflow-y: auto;
		overflow-x: hidden;
	}
	.file-list-items::-webkit-scrollbar {
		width: 14px;
	}

	.file-list-items::-webkit-scrollbar-track {
		background: var(--color-primary);
	}

	.file-list-items::-webkit-scrollbar-thumb {
		background-color: var(--color-secondary);
		border-radius: 14px;
		border: 3px solid var(--color-primary);
	}

	.file-list-items::-webkit-scrollbar-thumb:hover {
		border: 1px solid var(--color-primary);
	}
</style>
