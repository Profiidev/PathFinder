<script lang="ts">
	import { invoke } from '@tauri-apps/api/tauri';
	import FileList from '$lib/components/FileList.svelte';
	import { FileType } from '$lib/types';
	import type { FileData } from '$lib/types';
	import Topbar from '$lib/components/Topbar.svelte';
	import Sidebar from '$lib/components/Sidebar.svelte';
	import Bottombar from '$lib/components/Bottombar.svelte';
	import Settings from '$lib/components/Settings.svelte';
	import { settings, loadedFiles, settingsEnabled } from '$lib/stores';

	let testList: string[] = [];
	let input = '';
	let count = 0;
	let path = 'C:';

	const search = async () => {
		const res = await invoke('search_partial', {
			name: input,
			path,
			searchType: 'contains_no_type',
			isDir: false
		}).catch((err) => {
			console.log(err);
		});
		let data = res as { is_dir: boolean; name: string }[];
		if (data == null) {
			testList = [];
			count = 0;
			return;
		}
		testList = data.map((item) => item.name);
		count = testList.length;
		console.log(testList);
	};
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
				<FileList />
			</div>
		</div>
		<div class="bottombar-container">
			<Bottombar />
		</div>
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
