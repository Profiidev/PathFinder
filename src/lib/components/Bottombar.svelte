<script lang="ts">
	import Svg from '$lib/components/Svg.svelte';
	import { settings, loadedFiles, selectedFiles, settingsEnabled } from '$lib/stores';
	import { ViewType } from '$lib/types';

	const handleAppearanceSelectionClick = (viewType: ViewType) => {
		$settings.appearance.viewType = viewType;
	};
</script>

<div class="bottombar">
	<div class="bottombar-left" style="min-width: {$settings.sidebar.width}em;">
		<button class="settings reset-button" on:click={() => {$settingsEnabled = true}}>
			<Svg
				svgData={{ data: { path: '/svgs/ui/settings.svg', colors: [] }, width: 20, height: 20 }}
			/>
		</button>
	</div>
	<div class="bottombar-right" style="width: calc(100% - {$settings.sidebar.width}em);">
		<div class="bottombar-file-list-info">
			{#if $loadedFiles.length > 0}
				<span>{$loadedFiles.length} Element{$loadedFiles.length > 1 ? 's' : ''}</span>
				{#if $selectedFiles.files.length > 0}
					<span>&nbsp;- {$selectedFiles.files.length} selected</span>
				{/if}
			{/if}
		</div>
		<div class="appearance-selection">
			<button
				class="appearance-selection-grid reset-button {$settings.appearance.viewType ===
				ViewType.GRID
					? 'appearance-selection-selected'
					: ''}"
				on:click={() => handleAppearanceSelectionClick(ViewType.GRID)}
			>
				<Svg
					svgData={{
						data: {
							path: '/svgs/ui/grid_view.svg',
							colors: []
						},
						width: 20,
						height: 20
					}}
				/>
			</button>
			<button
				class="appearance-selection-list reset-button {$settings.appearance.viewType ===
				ViewType.LIST
					? 'appearance-selection-selected'
					: ''}"
				on:click={() => handleAppearanceSelectionClick(ViewType.LIST)}
			>
				<Svg
					svgData={{
						data: {
							path: '/svgs/ui/list_view.svg',
							colors: []
						},
						width: 20,
						height: 20
					}}
				/>
			</button>
		</div>
	</div>
</div>

<style>
	.bottombar {
		display: flex;
		flex-direction: row;
		align-items: center;
		height: 100%;
		width: 100%;
		background-color: var(--color-primary-dark);
		color: var(--color-secondary-light);
	}

	.bottombar-left {
		display: flex;
		flex-direction: row;
		align-items: center;
		height: 100%;
		border-right: 1px solid var(--color-secondary-dark);
		background-color: var(--color-primary-dark);
		z-index: 1;
	}

	.settings {
		display: flex;
		flex-direction: row;
		align-items: center;
		justify-content: center;
		height: 100%;
		width: 2em;
		border-radius: 0.5em;
	}

	.settings:hover {
		background-color: var(--color-secondary);
	}

	.bottombar-right {
		display: flex;
		flex-direction: row;
		align-items: center;
		justify-content: flex-end;
		height: 100%;
	}

	.appearance-selection {
		display: flex;
		flex-direction: row;
		align-items: center;
		justify-content: center;
		height: 100%;
		width: 3.5em;
		background-color: var(--color-primary-dark);
		border-left: 1px solid var(--color-secondary-dark);
		user-select: none;
	}

	.appearance-selection-list {
		display: flex;
		flex-direction: row;
		align-items: center;
		justify-content: center;
		border-radius: 0 0.5em 0.5em 0;
		height: 100%;
		width: 1.7em;
	}

	.appearance-selection-grid:hover,
	.appearance-selection-list:hover {
		background-color: var(--color-secondary);
	}

	.appearance-selection-grid {
		display: flex;
		flex-direction: row;
		align-items: center;
		justify-content: center;
		border-radius: 0.5em 0 0 0.5em;
		height: 100%;
		width: 1.7em;
		margin-right: 0.1em;
	}

	.appearance-selection-selected {
		background-color: var(--color-secondary);
	}

	.bottombar-file-list-info {
		display: flex;
		flex-direction: row;
		align-items: center;
		justify-content: center;
		height: 100%;
		max-width: calc(100% - 4.5em);
		padding: 0 0.5em;
		border-left: 1px solid var(--color-secondary-dark);
		user-select: none;
	}

	.bottombar-file-list-info span {
		white-space: pre;
		overflow: hidden;
		text-overflow: ellipsis;
	}
</style>
