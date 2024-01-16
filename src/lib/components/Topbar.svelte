<script lang="ts">
	import Svg from '$lib/components/Svg.svelte';
	import { settings } from '$lib/stores';

	let pathVisualizerVisible = true;
	let pathInput: HTMLInputElement;
	let pathWidth: number = 0;

	$: pathWidth = pathInput?.offsetWidth;

	const focusLoss = (e: Event) => {
		$settings.currentPath = $settings.currentPath.trim().replace(/\\/g, '/');
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
			}
		}
	};

	let pathParts: string[] = [];

	$: pathParts = $settings.currentPath.split('/').filter((part) => part !== '');
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
	<div class="topbar-path-container" bind:clientWidth={pathWidth}>
		<input
			type="text"
			placeholder="Path"
			class="topbar-input topbar-path"
			on:blur={focusLoss}
			on:focus={focusGain}
			on:keydown={keydown}
			bind:value={$settings.currentPath}
			bind:this={pathInput}
		/>
	</div>
	<div class="topbar-spacing"></div>
	<input type="text" placeholder="Search" class="topbar-input topbar-search" on:keydown={keydown} />

	<div class="path-visualizer" style="max-width: {pathWidth}px;">
		{#if pathParts.length > 0 && pathVisualizerVisible}
			{#each pathParts as part, index}
				<div class="path-visualizer-part" style="{index === 0 ? "padding-left: .5em;" : ""}">
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

	.topbar-path-container {
		width: 100%;
	}

	.topbar-path {
		margin-left: 0.5em;
		border-radius: 0.25em;
		height: 2.2em;
		width: calc(100% - 1.5em);
		min-width: 2.5em;
		padding: 0 0.5em;
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
		background: url('/svgs/ui/search.svg') no-repeat scroll right center;
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
		padding: 0 .2em;
		min-width: .6em;
	}

	.path-visualizer-part span {
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
