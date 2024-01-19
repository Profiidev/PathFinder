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
	let currentFetch = 0;
	let currentOverlayFetch = 0;

	$: svgData, overlayData,
		fetchSvg();

	const fetchSvg = () => {
		currentFetch++;
		fetchSvgHtml(currentFetch);

		if (overlayData.data.path !== '') {
			currentOverlayFetch++;
			fetchOverlaySvgHtml(currentOverlayFetch);
		}
	}

	const fetchSvgHtml = (fetchNumber: number) => {
		fetch(svgData.data.path)
			.then((res) => res.text())
			.then((data) => {
				svgData.data.colors?.forEach((color) => {
					data = data.replaceAll(new RegExp(color.key, 'g'), color.color);
				});
				if(fetchNumber === currentFetch)
					svg = data;
			});
	}

	const fetchOverlaySvgHtml = (fetchNumber: number) => {
		fetch(overlayData.data.path)
			.then((res) => res.text())
			.then((data) => {
				overlayData.data.colors.forEach((color) => {
					data = data.replaceAll(new RegExp(color.key, 'g'), color.color);
				});
				if(fetchNumber === currentOverlayFetch)
					overlaySvg = data;
			});
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
