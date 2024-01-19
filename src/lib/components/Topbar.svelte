<script lang="ts">
	import Svg from '$lib/components/Svg.svelte';
	import { windowSettings, pathHistory, settings } from '$lib/stores';
	import { getTextColor } from '$lib/utils/theme';
	import type { SvgColor } from '$lib/types';
	import { loadFiles } from '$lib/backend/files';
	import { search } from '$lib/backend/search';
	import { getPrimaryColor } from '$lib/utils/icon_resolver';

	let pathVisualizerVisible = true;
	let pathInput: HTMLInputElement;
	let searchInput: HTMLInputElement;
	let pathWidth: number = 0;

	$: pathWidth = pathInput?.offsetWidth;

	const focusLoss = (e: Event) => {
		$windowSettings.currentPath = $windowSettings.currentPath.trim().replace(/\\/g, '/');
		if ($windowSettings.currentPath === '') $windowSettings.currentPath = 'C:/';
		if ($windowSettings.currentPath[$windowSettings.currentPath.length - 1] !== '/')
			$windowSettings.currentPath += '/';
		pathVisualizerVisible = true;
	};

	const focusGain = (e: Event) => {
		pathVisualizerVisible = false;
		pathInput.select();
	};

	const keydown = (e: Event) => {
		if (e instanceof KeyboardEvent) {
			if (e.key === 'Enter') {
				pathInput.blur();
				searchInput.blur();
			}
		}
	};

	let pathParts: string[] = [];
	let iconColors: SvgColor[] = [];

	$: pathParts = $windowSettings.currentPath.split('/').filter((part) => part !== '');
	$: iconColors = [{ key: '#FFFFFF', color: getTextColor($settings.appearance.theme) }];

	const gotToParent = () => {
		if (pathParts.length > 0) {
			$windowSettings.currentPath =
				$windowSettings.currentPath
					.split('/')
					.filter((p) => p !== '')
					.slice(0, -1)
					.join('/') + '/';
			if ($windowSettings.currentPath === '/') $windowSettings.currentPath = 'C:/';
		}
	};

	const goToPathPart = (index: number) => {
		if (index < pathParts.length) {
			$windowSettings.currentPath =
				$windowSettings.currentPath
					.split('/')
					.filter((p) => p !== '')
					.slice(0, index + 1)
					.join('/') + '/';
		}
	};

	const goBack = () => {
		if ($pathHistory.currentIndex > 0) {
			$pathHistory.historyUpdated = true;
			$pathHistory.currentIndex--;
			$windowSettings.currentPath = $pathHistory.paths[$pathHistory.currentIndex];
		}
	};

	const goForward = () => {
		if ($pathHistory.currentIndex < $pathHistory.paths.length - 1) {
			$pathHistory.historyUpdated = true;
			$pathHistory.currentIndex++;
			$windowSettings.currentPath = $pathHistory.paths[$pathHistory.currentIndex];
		}
	};

	let searchValue: string = '';
	let searching: boolean = false;
	let oldUseRegex: boolean = false;
	let oldCaseSensitive: boolean = false;

	$: searchValue, updatedSearch(false, false, false);
	$: $windowSettings.useRegex, updatedSearch(false, true, false);
	$: $windowSettings.caseSensitive, updatedSearch(false, false, true);

	const updatedSearch = (force: boolean, regex: boolean, caseSensitive: boolean) => {
		let newUseRegex = $windowSettings.useRegex;
		let newCaseSensitive = $windowSettings.caseSensitive;
		if (searchValue !== '') {
			if (searching || (regex && newUseRegex !== oldUseRegex) || force || (caseSensitive && newCaseSensitive !== oldCaseSensitive)) {
				oldUseRegex = newUseRegex;
				oldCaseSensitive = newCaseSensitive;
				search(searchValue);
			}
		} else {
			loadFiles();
		}
	};

	const onSearchFocus = () => {
		searching = true;
	};

	const onSearchBlur = () => {
		searching = false;
		updatedSearch(true, false, false);
	};
</script>

<div class="topbar">
	<div class="topbar-left">
		<button class="topbar-button reset-button" on:click={() => goBack()}>
			<Svg
				svgData={{
					data: { path: '/svgs/arrow/arrow_left.svg', colors: iconColors },
					width: 25,
					height: 25
				}}
			/>
		</button>
		<button class="topbar-button reset-button" on:click={() => goForward()}>
			<Svg
				svgData={{
					data: { path: '/svgs/arrow/arrow_right.svg', colors: iconColors },
					width: 25,
					height: 25
				}}
			/>
		</button>
		<button class="topbar-button reset-button" on:click={() => gotToParent()}>
			<Svg
				svgData={{
					data: { path: '/svgs/arrow/arrow_up.svg', colors: iconColors },
					width: 25,
					height: 25
				}}
			/>
		</button>
		<button class="topbar-button reset-button" on:click={() => loadFiles()}>
			<Svg
				svgData={{
					data: { path: '/svgs/arrow/arrow_repeat.svg', colors: iconColors },
					width: 18.75,
					height: 18.75
				}}
			/>
		</button>
	</div>
	<div class="topbar-path-container" bind:clientWidth={pathWidth}>
		<input
			type="text"
			placeholder="Path"
			class="topbar-input topbar-path"
			on:blur={focusLoss}
			on:focus={focusGain}
			on:keydown={keydown}
			bind:value={$windowSettings.currentPath}
			bind:this={pathInput}
		/>
	</div>
	<div class="topbar-spacing"></div>
	<div class="topbar-search-container">
		<input
			type="text"
			placeholder="Search"
			class="topbar-input topbar-search"
			style="color: var(--color-text);"
			on:blur={onSearchBlur}
			on:focus={onSearchFocus}
			on:keydown={keydown}
			bind:this={searchInput}
			bind:value={searchValue}
		/>
		<button
			class="regex-icon reset-button"
			on:click={() => ($windowSettings.useRegex = !$windowSettings.useRegex)}
		>
			<Svg
				svgData={{
					data: {
						path: '/svgs/ui/regex.svg',
						colors: !$windowSettings.useRegex
							? iconColors
							: [{ key: 'FFFFFF', color: getPrimaryColor($settings.appearance.iconTheme) }]
					},
					width: 18.75,
					height: 18.75
				}}
			/>
		</button>
		<button
			class="case-icon reset-button"
			on:click={() => ($windowSettings.caseSensitive = !$windowSettings.caseSensitive)}
		>
			<Svg
				svgData={{
					data: {
						path: '/svgs/ui/char_case.svg',
						colors: !$windowSettings.caseSensitive
							? iconColors
							: [{ key: 'FFFFFF', color: getPrimaryColor($settings.appearance.iconTheme) }]
					},
					width: 18.75,
					height: 18.75
				}}
			/>
		</button>
	</div>
	<div class="path-visualizer" style="max-width: {pathWidth}px;">
		{#if pathParts.length > 0 && pathVisualizerVisible}
			{#each pathParts as part, index}
				<button
					class="path-visualizer-part reset-button"
					style={index === 0 ? 'padding-left: .2em; margin-left: .3em;' : ''}
					on:click={() => goToPathPart(index)}
				>
					<span>{part}</span>
				</button>
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
</div>

<style>
	.topbar {
		display: flex;
		flex-direction: row;
		align-items: center;
		background-color: var(--color-primary-dark);
		color: var(--color-text);
		padding: 0 0.2em;
		height: 100%;
		width: 100%;
		font-size: 1em;
	}

	.topbar-left {
		display: flex;
		flex-direction: row;
		align-items: center;
		justify-content: flex-start;
	}

	.topbar-button {
		display: flex;
		flex-direction: row;
		align-items: center;
		justify-content: center;
		color: var(--color-text);
		padding: 0.5em;
		margin: 0 0.25em;
		border-radius: 0.25em;
		width: 2.2em;
		height: 2.2em;
		user-select: none;
	}

	.topbar-button:hover {
		background-color: var(--color-secondary);
	}

	.topbar-path-container {
		width: 100%;
	}

	.topbar-path {
		margin-left: 0.5em;
		border-radius: 0.25em;
		height: 2.2em;
		width: calc(100% - 0.5em);
		min-width: 2.5em;
		padding: 0 0.5em;
		box-sizing: border-box;
	}

	.topbar-spacing {
		min-width: 0.5em;
		height: 100%;
		background-color: var(--color-primary-dark);
		z-index: 1;
	}

	.topbar-search {
		margin-right: 0.6em;
		border-radius: 0.25em;
		height: 2.2em;
		min-width: 14.25em;
		padding: 0 1.7em 0 0.5em;
		background-size: 1.2em;
		background-position: calc(100% - 0.35em);
		z-index: 1;
	}

	.topbar-input {
		background-color: var(--color-secondary-dark);
		color: var(--color-secondary-dark);
		border: none;
		outline: none;
		font-size: 1em;
	}

	.topbar-input::placeholder {
		color: var(--color-secondary-light);
	}

	.topbar-input:focus {
		background-color: var(--color-primary-dark);
		border: 1px solid var(--color-secondary);
		border-bottom: 1px solid var(--color-accent);
		color: var(--color-text);
	}

	.regex-icon {
		position: fixed;
		display: flex;
		flex-direction: row;
		align-items: center;
		justify-content: center;
		top: 1.1em;
		right: 2.2em;
	}

	.case-icon {
		position: fixed;
		display: flex;
		flex-direction: row;
		align-items: center;
		justify-content: center;
		top: 1.15em;
		right: 0.7em;
	}

	.path-visualizer {
		position: fixed;
		display: flex;
		flex-direction: row;
		align-items: center;
		height: 2.2em;
		margin-left: 0.2em;
		user-select: none;
		left: 11.3em;
		border-radius: 0.25em;
	}

	.path-visualizer-part {
		display: flex;
		flex-direction: row;
		align-items: center;
		justify-content: center;
		height: 70%;
		border-radius: 0.25em;
		cursor: pointer;
		padding: 0 0.2em;
		min-width: 0.6em;
	}

	.path-visualizer-part span {
		font-size: 0.9em;
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
		height: 70%;
		border-radius: 0.25em;
		cursor: pointer;
		min-width: 0.95em;
	}

	.path-visualizer-part:hover,
	.path-visualizer-split:hover {
		background-color: var(--color-secondary);
	}
</style>
