<script lang="ts">
	import Svg from '$lib/components/Svg.svelte';

	let svgStyle =
		'display: flex; flex-direction: row; align-items: center; justify-content: center;';
	let pathValue = '';
	let pathParts: string[] = [];

	const focusLoss = (e: Event) => {
		pathValue = pathValue.replaceAll('\\', '/');
		pathParts = pathValue.split('/');
	};

</script>

<div class="topbar">
	<div class="topbar-left">
		<div class="topbar-button">
			<Svg src="/svgs/ui/arrow_left.svg" width={25} height={25} style={svgStyle} />
		</div>
		<div class="topbar-button">
			<Svg src="/svgs/ui/arrow_right.svg" width={25} height={25} style={svgStyle} />
		</div>
		<div class="topbar-button">
			<Svg src="/svgs/ui/arrow_up.svg" width={25} height={25} style={svgStyle} />
		</div>
		<div class="topbar-button">
			<Svg src="/svgs/ui/arrow_repeat.svg" width={18.75} height={18.75} style={svgStyle} />
		</div>
	</div>
	<input
		type="text"
		placeholder="Path"
		class="topbar-input topbar-path"
		on:blur={focusLoss}
		bind:value={pathValue}
	/>
	<input type="text" placeholder="Search" class="topbar-input topbar-search" />
</div>

<style>
	.topbar {
		display: flex;
		flex-direction: row;
		align-items: center;
		background-color: var(--color-primary);
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
		background-color: var(--color-primary);
		color: var(--color-text);
		padding: 0.5em;
		margin: 0 0.25em;
		border-radius: 0.25em;
		width: 1.2em;
		height: 1.2em;
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
		background-color: var(--color-tertiary);
		color: var(--color-text);
		border: none;
		outline: none;
		font-size: 1em;
	}

	.topbar-input::placeholder {
		color: var(--color-quaternary);
	}

	.topbar-input:focus {
		background-color: var(--color-primary);
		border: 1px solid var(--color-secondary);
		border-bottom: 1px solid var(--color-accent);
	}
</style>
