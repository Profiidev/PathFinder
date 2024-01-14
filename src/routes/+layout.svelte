<script lang="ts">
	import { pressedKeys } from '$lib/stores';

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
</script>

<svelte:body on:keydown={keyDownHandler} on:keyup={keyUpHandler} />
<slot />

<style>
	:global(:root) {
		font-family: 'JetBrains Mono', monospace;

		--color-primary: rgb(28, 29, 38);
		--color-secondary: rgb(52, 52, 61);
		--color-tertiary: rgb(41, 41, 49);

		--color-accent: rgb(28, 153, 255);

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
</style>
