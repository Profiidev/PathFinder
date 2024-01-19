<script lang="ts">
	import type { FileData, FileListHeader } from '$lib/types';
	import { SortType, FileType } from '$lib/types';
	import FileListEntry from './FileListEntry.svelte';
	import FileListHeaderEntry from './FileListHeaderEntry.svelte';
	import { settings, pressedKeys, selectedFiles, loadedFiles, windowSettings } from '$lib/stores';
	import { fileListHeaderMinWidth, zoomStep, maxZoom, minZoom } from '$lib/utils/constants';
	import LazyLoder from './LazyLoder.svelte';
	import HeaderContext from './HeaderContext.svelte';

	let files: FileData[] = [];
	$: files = $loadedFiles;

	$: {
		if(!$settings.fileList.showHiddenFiles) {
			files = files.filter((file) => !file.hidden);
		}
	}

	$: files = files.sort((a, b) => {
		let result = $windowSettings.sortAscending ? 1 : -1;
		switch ($windowSettings.sortType) {
			case SortType.NAME:
				if(a.type === b.type) {
					result *= a.name.localeCompare(b.name, undefined, { numeric: true, sensitivity: 'base' });
				} else {
					result = a.type === FileType.DIRECTORY ? -1 : 1;
				}
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
			case SortType.PERMISSIONS:
				result *= a.permissions === b.permissions ? 0 : a.permissions ? 1 : -1;
				break;
			case SortType.TYPE:
				result *= a.type.localeCompare(b.type, undefined, { numeric: true, sensitivity: 'base' });
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
		if ($windowSettings.sortType === type) {
			$windowSettings.sortAscending = !$windowSettings.sortAscending;
		} else {
			$windowSettings.sortType = type;
			$windowSettings.sortAscending = true;
		}
	};

	let lastX = 0;
	let isResizing = false;
	let pxEmConversionFactor = 0;
	let currentType: SortType;
	let headers: FileListHeader[] = [];
	$: headers = $settings.fileList.fileListHeaders;

	const mouseDown = (e: MouseEvent, pxEmConversion: number, type: SortType) => {
		if (isResizing || e.button !== 0) return;
		lastX = e.clientX;
		isResizing = true;
		pxEmConversionFactor = pxEmConversion;
		currentType = type;
	};

	const mouseUpHandler = (e: MouseEvent) => {
		isResizing = false;
		//$settings.fileList.fileListHeaders = headers;
	};

	const mouseMoveHandler = (e: MouseEvent) => {
		if (!isResizing) return;

		const delta = e.clientX - lastX;
		lastX = e.clientX;
		let toAdd = Math.round(delta * pxEmConversionFactor * 100) / 100;
    
		let index = 0;
		headers.forEach((header, i) => {
			if (header.sortType === currentType) {
				index = i;
			}
		});
    headers[index].width = Math.max(fileListHeaderMinWidth, headers[index].width + toAdd);
	};

	let lastSingleClick = 0;

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
							.map((file) => file.path)
							.filter((f) => !data.files.includes(f))
					];
				}
			} else if ($pressedKeys.includes('control')) {
				if (selected) {
					data.files = data.files.filter((f) => f !== file.path);
				} else {
					data.files = [...data.files, file.path];
				}
				data.lastSelectedIndex = files.indexOf(file);
			} else if ($pressedKeys.includes('shift')) {
				if (data.lastSelectedIndex === -1) {
					data.files = files.slice(0, files.indexOf(file) + 1).map((file) => file.path);
				} else {
					const start = Math.min(data.lastSelectedIndex, files.indexOf(file));
					const end = Math.max(data.lastSelectedIndex, files.indexOf(file));
					data.files = files.slice(start, end + 1).map((file) => file.path);
				}
			} else {
				if (selected) {
					if (data.files.length === 1) {
						if(Date.now() - lastSingleClick < 500) {
							if(file.type === FileType.DIRECTORY) {
								$windowSettings.currentPath = file.path + '/';
							} else {
								//TODO open file
							}
						} else {
							//TODO change name
						}
					} else {
						data.files = [file.path];
					}
				} else {
					data.files = [file.path];
				}
				data.lastSelectedIndex = files.indexOf(file);
				lastSingleClick = Date.now();
			}
			return data;
		});
	};

	const scrollHandler = (e: Event) => {
		if(!$pressedKeys.includes('control')) return;

		const delta = (e as WheelEvent).deltaY * -zoomStep / 100;
		$windowSettings.zoom = Math.max(minZoom, Math.min(maxZoom, $windowSettings.zoom + delta));
	};

	let headerWidths: number[] = [];
	let totalWidth = 0;

	$: {
		totalWidth = headerWidths.reduce((a, b) => a + b, 0);
	}

	const clickHandler = () => {
		$selectedFiles.files = [];
	}

	let items: Element;
	let lastPath = '';

	$: $windowSettings.currentPath,
		resetScroll();

	const resetScroll = () => {
		let newPath = $windowSettings.currentPath;
		if(items && newPath !== lastPath) {
			items.scrollTop = 0;
			lastPath = newPath;
		}
	}

	let headerContextVisible = false;
	let headerContextX = 0;
	let headerContextY = 0;

	const headerContextHandler = (e: MouseEvent) => {
		e.preventDefault();
		e.stopPropagation();
		headerContextVisible = true;
		if(e.clientX + 14 * $windowSettings.zoom * 18 > window.innerWidth) {
			headerContextX = e.clientX - 14 * $windowSettings.zoom * 18;
		} else {
			headerContextX = e.clientX;
		}
		headerContextY = e.clientY;
	}
</script>

<svelte:window on:mouseup={mouseUpHandler} on:mousemove={mouseMoveHandler} />

<div class="file-list" style="font-size: {18 * $windowSettings.zoom}px;" on:wheel={scrollHandler}>
	<button class="file-list-header reset-button" on:contextmenu={headerContextHandler}>
		<div class="file-list-header-left" style="min-width: 2.75em;" bind:clientWidth={headerWidths[0]}></div>
		{#each headers as header, i}
			{#if header.active}
				<FileListHeaderEntry
					width={header.width}
					type={header.sortType}
					text={header.name}
					bind:headerWidth={headerWidths[i + 1]}
					{onSort}
					{mouseDown}
				/>
			{/if}
		{/each}
		<HeaderContext x={headerContextX} y={headerContextY} bind:visible={headerContextVisible} />
	</button>
	<button class="file-list-items scrollbar reset-button" on:click={clickHandler} bind:this={items}>
		{#each files as file}
			<LazyLoder height={3.15 * 18 * $windowSettings.zoom} width={totalWidth}>
				<FileListEntry {file} {onSelected} width={totalWidth} bind:headers={headers} />
			</LazyLoder>
		{/each}
	</button>
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
		align-items: flex-start;
		overflow-y: auto;
		overflow-x: hidden;
		height: 100%;
	}
</style>
