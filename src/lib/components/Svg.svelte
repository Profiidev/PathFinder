<script lang="ts">
	export let src: string;
	export let style: string = '';
	export let width: number = 50;
	export let height: number = 50;
</script>

<div style={style}>
	<img {src} alt="Svg" style="width: {width}px; height: {height}px;" />
</div>
