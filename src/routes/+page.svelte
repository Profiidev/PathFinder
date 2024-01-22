<script lang="ts">
	import FileList from '$lib/components/FileList.svelte';
	import Topbar from '$lib/components/Topbar.svelte';
	import Sidebar from '$lib/components/Sidebar.svelte';
	import Bottombar from '$lib/components/Bottombar.svelte';
	import Settings from '$lib/components/Settings.svelte';
	import { settings, settingsEnabled } from '$lib/stores';
	import FileContextMenu from '$lib/components/FileContextMenu.svelte';

	let fileContextVisible = false;
	let fileContextX = 0;
	let fileContextY = 0;
	let fileContextData = {};
</script>

<div class="container">
	{#if !$settingsEnabled}
		<div class="topbar-container">
			<Topbar />
		</div>
		<div class="content-container">
			<div class="sidebar-container" style="width: {$settings.sidebar.width}em">
				<Sidebar />
			</div>
			<div class="file-list-container" style="width: calc(100% - {$settings.sidebar.width}em)">
				<FileList bind:fileContextVisible bind:fileContextX bind:fileContextY bind:fileContextData />
			</div>
		</div>
		<div class="bottombar-container">
			<Bottombar />
		</div>
		<FileContextMenu bind:visible={fileContextVisible} bind:fileContextX bind:fileContextY bind:fileContextData />
	{:else}
		<Settings />
	{/if}
</div>

<style>
	.container {
		display: flex;
		flex-direction: column;
		align-items: left;
		background-color: var(--color-primary);
		margin: 0;
		padding: 0;
		height: 100vh;
		width: 100vw;
		overflow: hidden;
	}

	.topbar-container {
		height: 3.5em;
		min-height: 3.5em;
		width: 100%;
	}

	.content-container {
		display: flex;
		flex-direction: row;
		height: calc(100vh - 5.5em);
		width: 100%;
		border-top: 1px solid var(--color-secondary-dark);
	}

	.sidebar-container {
		height: 100%;
		width: 10em;
		min-width: 10em;
		border-right: 1px solid var(--color-secondary-dark);
	}

	.file-list-container {
		height: 100%;
	}

	.bottombar-container {
		height: 2em;
		min-height: 2em;
		width: 100%;
		border-top: 1px solid var(--color-secondary-dark);
	}
</style>
