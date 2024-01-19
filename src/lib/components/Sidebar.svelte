<script lang="ts">
	import SidebarEntry from '$lib/components/SidebarEntry.svelte';
	import SidebarSection from '$lib/components/SidebarSection.svelte';
	import Svg from '$lib/components/Svg.svelte';
	import { settings, locations, windowSettings } from '$lib/stores';
	import { getIconData, getSecondaryColor, getPrimaryColor } from '$lib/utils/icon_resolver';
	import { sidebarMinWidth } from '$lib/utils/constants';
	import { FileType } from '$lib/types';
	import { getUserName } from '$lib/backend/settings';

	let lastX = 0;
	let isResizing = false;
	let pxEmConversionFactor = 0;
	let sidebar: HTMLDivElement;

	const mouseDownHandler = (e: MouseEvent) => {
		if (isResizing) return;
		lastX = e.clientX;
		isResizing = true;
		pxEmConversionFactor = 1 / parseFloat(getComputedStyle(sidebar).fontSize);
	};

	const mouseUpHandler = () => {
		isResizing = false;
	};

	const mouseMoveHandler = (e: MouseEvent) => {
		if (!isResizing) return;
		const delta = e.clientX - lastX;
		lastX = e.clientX;
		let toAdd = Math.round(delta * pxEmConversionFactor * 100) / 100;

		if ($settings.sidebar.width + toAdd < sidebarMinWidth) {
			toAdd = sidebarMinWidth - $settings.sidebar.width;
		}
		$settings.sidebar.width += toAdd;
	};

	const locationClick = (path: string) => {
		$windowSettings.currentPath = path;
	}
</script>

<svelte:window on:mouseup={mouseUpHandler} on:mousemove={mouseMoveHandler} />

<div class="sidebar scrollbar">
	<div class="sidebar-content" bind:this={sidebar}>
		<div class="sidebar-top">
			<SidebarEntry text="Home" onClick={async () => ($windowSettings.currentPath = 'C:/Users/' + await getUserName() + '/')}>
				<Svg
					svgData={{
						data: getIconData(FileType.DIRECTORY, $settings.appearance.iconTheme),
						width: 20,
						height: 20
					}}
					overlayData={{
						data: { path: '/svgs/folder/home.svg', colors: [{ key: 'FFFFFF', color: getSecondaryColor($settings.appearance.iconTheme) }] },
						width: 12,
						height: 12
					}}
				/>
			</SidebarEntry>
			<SidebarEntry svgData={{ path: '/svgs/ui/network.svg', colors: [{ key: 'FFFFFF', color: getPrimaryColor($settings.appearance.iconTheme) }] }} text="Network" />
		</div>
		<SidebarSection index={0} text="Pinned">
			{#each $settings.sidebar.pinnedPaths as pinned}
				<SidebarEntry
					svgData={getIconData(pinned.type, $settings.appearance.iconTheme)}
					text={pinned.name}
					onClick={() => ($windowSettings.currentPath = pinned.path + '/')}
				/>
			{/each}
		</SidebarSection>
		<SidebarSection index={1} text="Devices">
			<SidebarEntry svgData={{ path: '/svgs/ui/pc.svg', colors: [] }} text="This PC" />
			{#each $settings.sidebar.devices as device}
				<SidebarEntry svgData={{ path: '/svgs/ui/pc.svg', colors: [] }} text={device.name} />
			{/each}
		</SidebarSection>
		<SidebarSection index={2} text="Locations">
			{#each $locations as location}
				<SidebarEntry
					svgData={getIconData(location.type, $settings.appearance.iconTheme)}
					text={location.name}
					onClick={() => locationClick(location.path)}
				/>
			{/each}
		</SidebarSection>
		<SidebarSection index={3} text="Tags">
			{#each $settings.sidebar.tags as tag}
				<SidebarEntry text={tag.name}>
					<div
						style="background-color: {tag.color}; width: .8em; height: .8em; border-radius: 0.5em;"
					></div>
				</SidebarEntry>
			{/each}
		</SidebarSection>
	</div>
	<button class="sidebar-resizer reset-button" on:mousedown={mouseDownHandler}></button>
</div>

<style>
	.sidebar {
		display: flex;
		flex-direction: row;
		background-color: var(--color-primary-dark);
		color: var(--color-secondary-light);
		height: 100%;
		width: 100%;
		font-size: 1em;
		overflow-x: hidden;
		overflow-y: auto;
	}

	.sidebar-content {
		display: flex;
		flex-direction: column;
		width: calc(100% - 0.25em);
		height: 100%;
	}

	.sidebar-resizer {
		width: 0.25em;
		height: 100%;
		background-color: var(--color-primary-dark);
		cursor: col-resize;
	}

	.sidebar-top {
		display: flex;
		flex-direction: column;
		width: calc(100% - 1em);
		margin: 0.2em;
	}

	.sidebar::-webkit-scrollbar-track {
		background-color: var(--color-primary-dark);
	}

	.sidebar::-webkit-scrollbar-thumb {
		border-color: var(--color-primary-dark);
		color: #002ab6;
	}
</style>
