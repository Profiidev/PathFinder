<script lang="ts">
	import { locations, settings } from '$lib/stores';
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

	let tab = 0;
	let tabs = ['General', 'Locations', 'Appearance', 'About'];

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
            finishLocationUpdate(res, 'Added');
					});
				} else {
          locationWorking = false;
        }
			});
	};

  const finishLocationUpdate = (res: unknown, text: string) => {
    if(!res) {
      locationWorking = false;
      locationModalText = 'Error while adding Location';
      locationModalOpen = true;
      return;
    }

    updateLocations();
    locationWorking = false;
    locationModalText = text + ' Location successfully';
    locationModalOpen = true;
  }
</script>

<div class="settings">
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
	<div class="settings-content">
		{#if tab === 0}{/if}
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
		{#if tab === 2}{/if}
		{#if tab === 3}{/if}
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

	.tabs {
		display: flex;
		flex-direction: column;
		width: 9em;
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
</style>
