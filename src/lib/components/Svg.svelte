<script lang="ts">
	import type { SvgColor, SvgInfo } from '$lib/types';

	export let svgData: SvgInfo = {
		data: {
			path: '',
			colors: []
		},
		width: 50,
		height: 50
	};
	export let overlayData: SvgInfo = {
		data: {
			path: '',
			colors: []
		},
		width: 50,
		height: 50
	};

	let svg: string = '';
	let overlaySvg: string = '';

	$: {
		fetch(svgData.data.path)
			.then((res) => res.text())
			.then((data) => {
				svgData.data.colors.forEach((color) => {
					data = data.replace(new RegExp(color.key, 'g'), color.color);
				});
				svg = data;
			});

		if (overlayData.data.path !== '') {
			fetch(overlayData.data.path)
				.then((res) => res.text())
				.then((data) => {
					overlayData.data.colors.forEach((color) => {
						data = data.replace(new RegExp(color.key, 'g'), color.color);
					});
					overlaySvg = data;
				});
		}
	}
</script>

<div
	class="icon-svg"
	style="min-width: {svgData.width}px; min-height: {svgData.height}px; width: {svgData.width}px; height: {svgData.height}px;"
>
	{@html svg}
	<div
		class="icon-svg-overlay"
		style="min-width: {overlayData.width}px; min-height: {overlayData.height}px; width: {overlayData.width}px; height: {overlayData.height}px;"
	>
		{@html overlaySvg}
	</div>
</div>

<style>
	.icon-svg {
		display: flex;
		flex-direction: row;
		align-items: center;
		justify-content: center;
		pointer-events: none;
		user-select: none;
	}

	.icon-svg-overlay {
		position: fixed;
		display: flex;
		flex-direction: row;
		align-items: center;
		justify-content: center;
		pointer-events: none;
		user-select: none;
	}
</style>
