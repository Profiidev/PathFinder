<script lang="ts">
	import { pathHistory, pressedKeys, settings, windowSettings } from '$lib/stores';
	import {
		getPrimaryColor as getAccentColor,
		getSecondaryColor as getAccentColorDark
	} from '$lib/utils/icon_resolver';
	import {
		getPrimaryColor,
		getPrimaryDarkColor,
		getSecondaryColor,
		getSecondaryLightColor,
		getSecondaryDarkColor,
		getTextColor
	} from '$lib/utils/theme';
	import { load } from '$lib/start';
	import { onMount } from 'svelte';
	import { getUserName, saveSettings } from '$lib/backend/settings';
	import { loadFiles } from '$lib/backend/files';

	const keyDownHandler = (e: KeyboardEvent) => {
		pressedKeys.update((keys) => {
			if (!keys.includes(e.key.toLowerCase())) {
				return [...keys, e.key.toLowerCase()];
			} else {
				return keys;
			}
		});
	};

	const keyUpHandler = (e: KeyboardEvent) => {
		pressedKeys.update((keys) => {
			return keys.filter((key) => key !== e.key.toLowerCase());
		});
	};

	const mouseKeyDownHandler = (e: MouseEvent) => {
		pressedKeys.update((keys) => {
			if (!keys.includes('mouse' + e.button)) {
				return [...keys, 'mouse' + e.button];
			} else {
				return keys;
			}
		});
	};

	const mouseKeyUpHandler = (e: MouseEvent) => {
		pressedKeys.update((keys) => {
			return keys.filter((key) => key !== 'mouse' + e.button);
		});
	};

	$: $settings.appearance.iconTheme,
		document.body.style.setProperty(
			'--color-accent',
			'#' + getAccentColor($settings.appearance.iconTheme)
		),
		document.body.style.setProperty(
			'--color-accent-dark',
			'#' + getAccentColorDark($settings.appearance.iconTheme)
		);

	$: $settings.appearance.theme,
		document.body.style.setProperty('--color-primary', getPrimaryColor($settings.appearance.theme)),
		document.body.style.setProperty(
			'--color-primary-dark',
			getPrimaryDarkColor($settings.appearance.theme)
		),
		document.body.style.setProperty(
			'--color-secondary',
			getSecondaryColor($settings.appearance.theme)
		),
		document.body.style.setProperty(
			'--color-secondary-dark',
			getSecondaryDarkColor($settings.appearance.theme)
		),
		document.body.style.setProperty(
			'--color-secondary-light',
			getSecondaryLightColor($settings.appearance.theme)
		),
		document.body.style.setProperty('--color-text', getTextColor($settings.appearance.theme));

	$: $settings, saveSettings();

	$: $windowSettings.currentPath, reloadFiles();

	const reloadFiles = () => {
		if ($windowSettings.currentPath[$windowSettings.currentPath.length - 1] === '/') {
			loadFiles();
			if (
				$pathHistory.historyUpdated ||
				!$settings.loaded ||
				($pathHistory.currentIndex !== -1 &&
					$windowSettings.currentPath === $pathHistory.paths[$pathHistory.currentIndex])
			) {
				$pathHistory.historyUpdated = false;
				return;
			}
			$pathHistory.currentIndex++;
			$pathHistory.paths = $pathHistory.paths.slice(0, $pathHistory.currentIndex);
			$pathHistory.paths.push($windowSettings.currentPath);
		}
	};

	onMount(() => {
		load();
	});
</script>

<svelte:window
	on:keydown={keyDownHandler}
	on:keyup={keyUpHandler}
	on:mousedown={mouseKeyDownHandler}
	on:mouseup={mouseKeyUpHandler}
/>
<slot />

<style>
	:global(:root) {
		font-family: 'JetBrains Mono', monospace;

		--color-primary: rgb(28, 29, 38);
		--color-primary-dark: rgb(13, 13, 22);
		--color-secondary: rgb(52, 52, 61);
		--color-secondary-dark: rgb(35, 35, 44);
		--color-secondary-light: rgb(146, 146, 160);

		--color-accent: #1d98ff;
		--color-accent-dark: #002ab6;

		--color-text: rgb(255, 255, 255);

		overflow: hidden;
		position: fixed;
		width: 100vw;
		height: 100vh;
	}

	:global(.reset-button) {
		background: transparent;
		color: inherit;
		font: inherit;
		outline: none;
		padding: 0;
		text-align: inherit;
		text-decoration: none;
		border: none;
		display: flex;
		flex-direction: row;
		align-items: center;
	}

	:global(.scrollbar::-webkit-scrollbar) {
		width: 10px;
	}

	:global(.scrollbar::-webkit-scrollbar-track) {
		background: var(--color-primary);
	}

	:global(.scrollbar::-webkit-scrollbar-thumb) {
		background-color: var(--color-secondary);
		border-radius: 14px;
		border: 4px solid var(--color-primary);
	}

	:global(.scrollbar::-webkit-scrollbar-thumb:hover) {
		border: 2px solid var(--color-primary);
	}
</style>
