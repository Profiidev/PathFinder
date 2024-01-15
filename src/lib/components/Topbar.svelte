<script lang="ts">
	import Svg from '$lib/components/Svg.svelte';
	import { settings } from '$lib/stores';

	const focusLoss = (e: Event) => {
		$settings.currentPath = $settings.currentPath.trim().replace(/\\/g, '/');
	};

	const keydown = (e: Event) => {
		if (e instanceof KeyboardEvent) {
			if (e.key === 'Enter') {
				(e.srcElement as HTMLElement).blur();
			}
		}
	};
</script>

<div class="topbar">
	<div class="topbar-left">
		<div class="topbar-button">
			<Svg svgData={{data: {path: '/svgs/arrow/arrow_left.svg', colors: []}, width: 25, height: 25}} />
		</div>
		<div class="topbar-button">
			<Svg svgData={{data: {path: '/svgs/arrow/arrow_right.svg', colors: []}, width: 25, height: 25}} />
		</div>
		<div class="topbar-button">
			<Svg svgData={{data: {path: '/svgs/arrow/arrow_up.svg', colors: []}, width: 25, height: 25}} />
		</div>
		<div class="topbar-button">
			<Svg svgData={{data: {path: '/svgs/arrow/arrow_repeat.svg', colors: []}, width: 18.75, height: 18.75}} />
		</div>
	</div>
	<input
		type="text"
		placeholder="Path"
		class="topbar-input topbar-path"
		on:blur={focusLoss}
		on:keydown={keydown}
		bind:value={$settings.currentPath}
	/>
	<input type="text" placeholder="Search" class="topbar-input topbar-search" on:keydown={keydown} />
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
		width: 1.2em;
		height: 1.2em;
		user-select: none;
	}

	.topbar-button:hover {
		background-color: var(--color-secondary);
	}

	.topbar-path {
		margin: 0 0.5em;
		border-radius: 0.25em;
		height: 2.2em;
		width: 100%;
		min-width: 2.5em;
		padding: 0 0.5em;
	}

	.topbar-search {
		margin-right: 0.6em;
		border-radius: 0.25em;
		height: 2.2em;
		min-width: 14.25em;
		padding: 0 1.7em 0 0.5em;
		background: url('/svgs/ui/search.svg') no-repeat scroll right center;
		background-size: 1.2em;
		background-position: calc(100% - 0.35em);
	}

	.topbar-input {
		background-color: var(--color-secondary-dark);
		color: var(--color-text);
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
	}
</style>
