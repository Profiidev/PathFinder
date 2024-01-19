<script>
	import { windowSettings } from '$lib/stores';
	import Svg from '$lib/components/Svg.svelte';

	export let index = 0;
	export let text = '';
</script>

<div class="sidebar-section">
	<button
		class="sidebar-header reset-button"
		on:click={() => {
			$windowSettings.expanded[index] = !$windowSettings.expanded[index];
		}}
	>
		<span>{text}</span>
		<Svg
			svgData={{
				data: {
					path: $windowSettings.expanded[index]
						? '/svgs/simple_arrow/simple_arrow_up.svg'
						: '/svgs/simple_arrow/simple_arrow_down.svg',
					colors: [{ key: 'FFFFFF', color: '9293a0' }]
				},
				width: 10,
				height: 10
			}}
		/>
	</button>
	{#if $windowSettings.expanded[index]}
		<div class="sidebar-section-entries">
			<slot />
		</div>
	{/if}
</div>

<style>
	.sidebar-section {
		display: flex;
		flex-direction: column;
		width: calc(100% - 1em);
		margin: 0.5em;
	}

	.sidebar-header {
		display: flex;
		flex-direction: row;
		align-items: center;
		justify-content: space-between;
		border-radius: 0.25em;
		user-select: none;
	}

	.sidebar-section span {
		font-size: 0.7em;
		user-select: none;
	}

	.sidebar-section-entries {
		display: flex;
		flex-direction: column;
		height: 100%;
		width: 100%;
		align-items: flex-start;
		margin-top: 0.1em;
	}
</style>
