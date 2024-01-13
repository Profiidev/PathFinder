<script lang="ts">
	export let src: string;
	export let className: string = '';
	export let width: number = 50;
	export let height: number = 50;
</script>

<div class={className}>
	<img {src} alt="Svg" style="width: {width}px; height: {height}px;" />
</div>
