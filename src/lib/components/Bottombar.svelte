<script lang="ts">
	import Svg from '$lib/components/Svg.svelte';
	import { settings } from '$lib/stores';
	import { ViewType } from '$lib/types';
	import { getIconData } from '$lib/utils/icon_resolver';
	import { FileType, IconTheme } from '$lib/types';

	const handleAppearanceSelectionClick = (viewType: ViewType) => {
		$settings.appearance.viewType = viewType;
	};

	let pathParts: string[] = [];

	$: pathParts = $settings.currentPath.split('/').filter((part) => part !== '');
</script>

<div class="bottombar">
	<div class="bottombar-left" style="min-width: {$settings.sidebar.width}em;">
		<button class="settings reset-button" on:click={() => {$settings.appearance.iconTheme = IconTheme.GREEN}}>
			<Svg
				svgData={{ data: { path: '/svgs/ui/settings.svg', colors: [] }, width: 20, height: 20 }}
			/>
		</button>
	</div>
	<div class="bottombar-right" style="width: calc(100% - {$settings.sidebar.width}em);">
		<div class="path-visualizer">
			{#if pathParts.length > 0}
				{#each pathParts as part, index}
					<div class="path-visualizer-part">
						{#if index !== 0}
							<Svg
								svgData={{
									data: getIconData(FileType.DIRECTORY, $settings.appearance.iconTheme),
									width: 15,
									height: 15
								}}
							/>
						{/if}
						<span>{part}</span>
					</div>
					{#if index !== pathParts.length - 1}
						<div class="path-visualizer-split">
							<Svg
								svgData={{
									data: {
										path: '/svgs/simple_arrow/simple_arrow_right.svg',
										colors: [{ key: 'FFFFFF', color: '9293a0' }]
									},
									width: 10,
									height: 10
								}}
							/>
						</div>
					{/if}
				{/each}
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

	.path-visualizer {
		display: flex;
		flex-direction: row;
		align-items: center;
		height: 100%;
		margin-left: 0.2em;
		user-select: none;
		width: calc(100% - 3.5em);
	}

	.path-visualizer-part {
		display: flex;
		flex-direction: row;
		align-items: center;
		justify-content: center;
		height: 80%;
		border-radius: 0.5em;
		cursor: pointer;
		padding-left: 0.2em;
		min-width: 1em;
	}

	.path-visualizer-part span {
		margin-left: 0.4em;
		font-size: 0.8em;
		width: 100%;
		overflow: hidden;
		text-overflow: ellipsis;
		white-space: nowrap;
	}

	.path-visualizer-split {
		display: flex;
		flex-direction: row;
		align-items: center;
		justify-content: center;
		height: 80%;
		border-radius: 0.5em;
		cursor: pointer;
		min-width: 0.95em;
	}

	.path-visualizer-part:hover,
	.path-visualizer-split:hover {
		background-color: var(--color-secondary);
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
</style>
