<script lang="ts">
	import { pressedKeys, settings } from '$lib/stores';
	import { getPrimaryColor, getSecondaryColor } from '$lib/utils/icon_resolver';

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

	$: $settings.appearance.iconTheme, 
		document.body.style.setProperty('--color-accent', "#" + getPrimaryColor($settings.appearance.iconTheme)),
		document.body.style.setProperty('--color-accent-dark', "#" + getSecondaryColor($settings.appearance.iconTheme)),
		console.log(document.body.style)
	
</script>

<svelte:window on:keydown={keyDownHandler} on:keyup={keyUpHandler} />
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
