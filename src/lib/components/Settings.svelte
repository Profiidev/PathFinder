<script lang="ts">
	import { locations, settings, settingsEnabled } from '$lib/stores';
	import Svg from './Svg.svelte';
	import {
		reindexLocation,
		removeLocation,
		updateLocations,
		addLocation
	} from '$lib/backend/location';
	import { getPrimaryColor } from '$lib/utils/icon_resolver';
	import { dialog } from '@tauri-apps/api';
	import Modal from './Modal.svelte';
	import { IconTheme, Theme } from '$lib/types';
	import { maxZoom, minZoom, zoomStep } from '$lib/utils/constants';
	import { loadFiles } from '$lib/backend/files';

	let tab = 0;
	let tabs = ['General', 'Locations', 'Appearance', 'Tags', 'About'];

	let locationModalOpen = false;
	let locationModalText = '';
	let locationWorking = false;

	const reloadLocation = (location: string) => {
		locationWorking = true;
		reindexLocation(location).then((res) => {
			finishLocationUpdate(res, 'Reloaded');
		});
	};

	const deleteLocation = (location: string) => {
		locationWorking = true;
		removeLocation(location).then((res) => {
			finishLocationUpdate(res, 'Removed');
		});
	};

	const newLocation = () => {
		locationWorking = true;
		dialog
			.open({
				directory: true,
				multiple: false
			})
			.then((result) => {
				if (result !== null && !(result instanceof Array)) {
					$locations.forEach((l) => {
						if (l.path.startsWith(result) || result.startsWith(l.path)) {
							locationWorking = false;
							locationModalText = 'Location already exists';
							locationModalOpen = true;
							return;
						}
					});

					addLocation(result).then((res) => {
						console.log(res);
						finishLocationUpdate(res, 'Added');
					});
				} else {
					locationWorking = false;
				}
			});
	};

	const finishLocationUpdate = (res: unknown, text: string) => {
		if (!res) {
			locationWorking = false;
			locationModalText = 'Error while adding Location';
			locationModalOpen = true;
			return;
		}

		updateLocations();
		locationWorking = false;
		locationModalText = text + ' Location successfully';
		locationModalOpen = true;
	};

	const closeSettings = () => {
		$settingsEnabled = false;
		loadFiles();
	}
</script>

<div class="settings">
	<div class="tabs-selection">
		<div class="tabs">
			{#each tabs as t, i}
				<button
					class="tab-button reset-button {tab === i ? 'tab-button-selected' : ''}"
					on:click={() => (tab = i)}
				>
					<span>
						{t}
						{#if tab === i}
							<div class="selected"></div>
						{/if}
					</span>
				</button>
			{/each}
		</div>
		<div class="tabs-bottom">
			<button class="close-button reset-button" on:click={closeSettings}>
				<Svg
					svgData={{
						data: {
							path: '/svgs/arrow/arrow_left.svg',
							colors: [{ key: 'FFFFFF', color: '9292a0' }]
						},
						width: 20,
						height: 20
					}}
				/>
			</button>
		</div>
	</div>
	<div class="settings-content">
		{#if tab === 0}
			<div class="settings-tab">
				<span>General</span>
			</div>
		{/if}
		{#if tab === 1}
			<Modal open={locationModalOpen} header="Location" height={9}>
				<div class="location-modal-text">
					<span>{locationModalText}</span>
					<div class="location-modal-button">
						<button class="reset-button" on:click={() => (locationModalOpen = false)}>Close</button>
					</div>
				</div>
			</Modal>
			<div class="settings-tab">
				<div class="locations-header">
					<span>Locations</span>
					<button
						class="add-location reset-button"
						on:click={newLocation}
						disabled={locationWorking}
					>
						<Svg
							svgData={{
								data: {
									path: '/svgs/ui/add.svg',
									colors: [
										{ key: 'FFFFFF', color: getPrimaryColor($settings.appearance.iconTheme) }
									]
								},
								width: 20,
								height: 20
							}}
						/>
					</button>
				</div>
				<div class="locations scrollbar">
					{#each $locations as l}
						<div class="location">
							<span>{l.name}</span>
							<div class="location-buttons">
								<button
									class="reload-location reset-button"
									on:click={() => reloadLocation(l.path)}
									disabled={locationWorking}
								>
									<Svg
										svgData={{
											data: {
												path: '/svgs/arrow/arrow_repeat.svg',
												colors: [{ key: 'FFFFFF', color: '9292a0' }]
											},
											width: 20,
											height: 20
										}}
									/>
								</button>
								<button
									class="delete-location reset-button"
									on:click={() => deleteLocation(l.path)}
									disabled={locationWorking}
								>
									<Svg
										svgData={{
											data: {
												path: '/svgs/ui/delete.svg',
												colors: [{ key: 'FFFFFF', color: 'c71212' }]
											},
											width: 20,
											height: 20
										}}
									/>
								</button>
							</div>
						</div>
					{/each}
				</div>
			</div>
		{/if}
		{#if tab === 2}
			<div class="settings-tab">
				<span>Appearance</span>
				<div class="appearance">
					<div class="appearance-section">
						<span>Zoom</span>
						<input
							type="range"
							min={minZoom}
							max={maxZoom}
							step={zoomStep}
							bind:value={$settings.appearance.zoom}
							style="--webkit-fill-percent: {(($settings.appearance.zoom - minZoom) /
								(maxZoom - minZoom)) *
								91.66 +
								4.16}%;"
						/>
					</div>
					<div class="appearance-section">
						<span>Theme</span>
						<div class="appearance-icon-theme-buttons">
							<button
								class="reset-button {$settings.appearance.theme === 'dark'
									? 'icon-theme-selected'
									: ''}"
								style="border-radius: .5em 0 0 .5em;"
								on:click={() => ($settings.appearance.theme = Theme.DARK)}
							>
								<span>Dark</span>
							</button>
							<button
								class="reset-button {$settings.appearance.theme === 'light'
									? 'icon-theme-selected'
									: ''}"
								style="border-radius: 0 .5em .5em 0;"
								on:click={() => ($settings.appearance.theme = Theme.LIGHT)}
							>
								<span>Light</span>
							</button>
						</div>
					</div>
					<div class="appearance-section">
						<span>Icon Theme</span>
						<div class="appearance-icon-theme-buttons">
							<button
								class="reset-button {$settings.appearance.iconTheme === IconTheme.BLUE
									? 'icon-theme-selected'
									: ''}"
								style="border-radius: .5em 0 0 .5em;"
								on:click={() => ($settings.appearance.iconTheme = IconTheme.BLUE)}
							>
								<span>Blue</span>
							</button>
							<button
								class="reset-button {$settings.appearance.iconTheme === IconTheme.YELLOW
									? 'icon-theme-selected'
									: ''}"
								on:click={() => ($settings.appearance.iconTheme = IconTheme.YELLOW)}
							>
								<span>Yellow</span>
							</button>
							<button
								class="reset-button {$settings.appearance.iconTheme === IconTheme.PURPLE
									? 'icon-theme-selected'
									: ''}"
								on:click={() => ($settings.appearance.iconTheme = IconTheme.PURPLE)}
							>
								<span>Purple</span>
							</button>
							<button
								class="reset-button {$settings.appearance.iconTheme === IconTheme.GREEN
									? 'icon-theme-selected'
									: ''}"
								on:click={() => ($settings.appearance.iconTheme = IconTheme.GREEN)}
							>
								<span>Green</span>
							</button>
							<button
								class="reset-button {$settings.appearance.iconTheme === IconTheme.RED
									? 'icon-theme-selected'
									: ''}"
								style="border-radius: 0 .5em .5em 0;"
								on:click={() => ($settings.appearance.iconTheme = IconTheme.RED)}
							>
								<span>Red</span>
							</button>
						</div>
					</div>
					<div class="appearance-section">
						<span>Show Hidden Files</span>
						<label>
							<input
								type="checkbox"
								bind:checked={$settings.fileList.showHiddenFiles}
							/>
						</label>
					</div>
					<div class="appearance-section">
						<span>Show File Extensions</span>
						<label>
							<input
								type="checkbox"
								bind:checked={$settings.fileList.showFileExtensions}
							/>
						</label>
					</div>
				</div>
			</div>
		{/if}
		{#if tab === 3}
			<div class="settings-tab">
				<span>Tags</span>
			</div>
		{/if}
		{#if tab === 4}
			<div class="settings-tab">
				<span>About</span>
			</div>
		{/if}
	</div>
</div>

<style>
	.settings {
		position: absolute;
		top: 0;
		left: 0;
		width: 100%;
		height: 100%;
		display: flex;
		flex-direction: row;
		color: var(--color-text);
		user-select: none;
	}

	.tabs-selection {
		display: flex;
		flex-direction: column;
		width: 11.75em;
		height: 100%;
		background-color: var(--color-primary-dark);
		border-right: 1px solid var(--color-secondary-dark);
	}

	.tabs {
		display: flex;
		flex-direction: column;
		width: calc(100% - 1em);
		height: 100%;
		background-color: var(--color-primary-dark);
		border-right: 1px solid var(--color-secondary-dark);
		padding-top: 0.5em;
		padding-left: 1em;
		border-right: 1px solid var(--color-secondary-dark);
	}

	.tab-button {
		display: flex;
		flex-direction: row;
		height: 1.5em;
		width: 8em;
		margin: 0.25em 0;
	}

	.tab-button span {
		height: 100%;
	}

	.selected {
		width: 100%;
		height: 0.125em;
		background-color: var(--color-accent);
		border-radius: 0.1em;
		box-sizing: border-box;
	}

	.tabs-bottom {
		display: flex;
		flex-direction: row;
		align-items: center;
		width: 100%;
		height: 2em;
		padding-right: 1em;
		border-top: 1px solid var(--color-secondary-dark);
	}

	.close-button {
		display: flex;
		flex-direction: row;
		align-items: center;
		justify-content: center;
		width: 2em;
		height: 2em;
		border-radius: 0.5em;
	}

	.close-button:hover {
		background-color: var(--color-secondary);
	}

	.settings-content {
		display: flex;
		flex-direction: column;
		width: 100%;
		height: 100%;
	}

	.settings-tab {
		display: flex;
		flex-direction: column;
		width: calc(100% - 2em);
		height: calc(100% - 2em);
		padding: 1em;
	}

	.settings-tab span {
		font-size: 1.5em;
	}

	.locations-header {
		display: flex;
		flex-direction: row;
		align-items: center;
		justify-content: space-between;
	}

	.add-location {
		width: 2em;
		height: 2em;
		border-radius: 0.25em;
		display: flex;
		align-items: center;
		justify-content: center;
		margin-right: 1em;
	}

	.locations {
		display: flex;
		flex-direction: column;
		margin: 1em 1em 0 1em;
		overflow-y: auto;
	}

	.location {
		display: flex;
		flex-direction: row;
		align-items: center;
		justify-content: space-between;
		padding: 1em;
		border: 1px solid var(--color-secondary);
		border-radius: 0.25em;
		margin-bottom: 0.5em;
	}

	.location span {
		font-size: 1em;
	}

	.location-buttons {
		display: flex;
		flex-direction: row;
		align-items: center;
	}

	.reload-location {
		width: 2em;
		height: 2em;
		border-radius: 0.25em;
		display: flex;
		align-items: center;
		justify-content: center;
		margin-right: 0.5em;
	}

	.delete-location {
		width: 2em;
		height: 2em;
		border-radius: 0.25em;
		display: flex;
		align-items: center;
		justify-content: center;
	}

	.delete-location:enabled:hover,
	.reload-location:enabled:hover,
	.add-location:enabled:hover {
		background-color: var(--color-secondary);
	}

	.location-modal-text {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		height: 100%;
		margin-top: 0.5em;
	}

	.location-modal-button {
		display: flex;
		align-items: center;
		justify-content: flex-end;
		width: 100%;
		margin-top: 1em;
	}

	.location-modal-button button {
		background-color: var(--color-accent);
		padding: 0.2em 0.5em;
		border-radius: 0.25em;
	}

	.appearance {
		display: flex;
		flex-direction: column;
		align-items: center;
		margin-top: 1em;
	}

	.appearance-section {
		display: flex;
		flex-direction: row;
		align-items: center;
		justify-content: space-between;
		margin-bottom: 1em;
		margin-left: 1.5em;
		width: calc(100% - 1.5em);
		height: 2em;
	}

	.appearance-section span {
		font-size: 1em;
		white-space: nowrap;
	}

	.appearance-icon-theme-buttons {
		display: flex;
		flex-direction: row;
		align-items: center;
		justify-content: space-between;
		margin-left: 1em;
	}

	.appearance-icon-theme-buttons button {
		padding: 0.2em 0.5em;
		border: 1px solid var(--color-secondary);
		width: 5em;
		display: flex;
		align-items: center;
		justify-content: center;
	}

	.appearance-icon-theme-buttons button:not(.icon-theme-selected):hover {
		background-color: var(--color-secondary);
	}

	.icon-theme-selected {
		background-color: var(--color-accent);
		border: 1px solid var(--color-accent) !important;
	}

	.appearance input[type='range'] {
		width: 12em;
		height: 1em;
	}

	.appearance input[type='range']::-webkit-slider-runnable-track {
		background: linear-gradient(
			to right,
			var(--color-accent) 0%,
			var(--color-accent) calc(var(--webkit-fill-percent) - 1%),
			var(--color-primary) calc(var(--webkit-fill-percent) + 1%),
			var(--color-primary) 100%
		);
		border: 1px solid var(--color-secondary);
		border-radius: 1em;
	}

	.appearance input[type='range']::-webkit-slider-thumb {
		-webkit-appearance: none;
		border-radius: 50%;
		background-color: var(--color-accent-dark);
	}

	input[type='checkbox'] {
		appearance: none;
		width: 4em;
		height: 2em;
		background-color: var(--color-primary);
		outline: none;
		cursor: pointer;
		position: relative;
		border-radius: 2em;
		border: .1em solid var(--color-secondary);
		transition: background-color 0.3s ease;
	}

	input[type='checkbox']:checked {
		background-color: var(--color-accent);
	}

	input[type='checkbox']:before {
		content: '';
		position: absolute;
		top: 0.15em;
		left: 0.15em;
		width: 1.5em;
		height: 1.5em;
		background-color: var(--color-text);
		border-radius: 50%;
		transition: all 0.3s ease;
	}

	input[type='checkbox']:checked:before {
		transform: translateX(2em);
	}
</style>
