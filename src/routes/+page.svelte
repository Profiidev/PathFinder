<script lang="ts">
	import { invoke } from '@tauri-apps/api/tauri';
	import FileList from '$lib/components/FileList.svelte';
	import { FileType } from '$lib/types';
	import type { FileData } from '$lib/types';
	import Topbar from '$lib/components/Topbar.svelte';
	import Sidebar from '$lib/components/Sidebar.svelte';
	import Bottombar from '$lib/components/Bottombar.svelte';
	import { settings } from '$lib/stores';
	import SvgBuilder from '$lib/components/Svg.svelte';

	let testList: string[] = [];
	let input = '';
	let count = 0;
	let path = 'C:/Users/benja/Documents/Coding/Apps/FileExplorer';

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
	};

	const addLocation = async () => {
		const res = await invoke('add_location', { location: input }).catch((err) => {
			console.log(err);
		});
		console.log(res);
		path = input;
	};

	const removeLocation = async () => {
		const res = await invoke('remove_location', { location: input }).catch((err) => {
			console.log(err);
		});
		console.log(res);
	};

	const reindexLocation = async () => {
		const res = await invoke('reindex_location', { location: input }).catch((err) => {
			console.log(err);
		});
		console.log(res);
	};

	const getTree = async () => {
		const res = await invoke('get_tree', { location: input }).catch((err) => {
			console.log(err);
		});
		let data = res as {
			root: { children: {}[]; data: { name: string; path: string; is_dir: boolean } };
		};
		testList = [];
		count = 0;
		if (data == null) {
			return;
		}
		testList.push(data.root.data.name);
		data.root.children.forEach((child) => {
			traverse(
				child as { children: {}[]; data: { name: string; path: string; is_dir: boolean } },
				1
			);
		});
		count++;
	};

	const traverse = (
		node: { children: {}[]; data: { name: string; path: string; is_dir: boolean } },
		i: number
	) => {
		testList.push('|--'.repeat(i) + node.data.name);
		node.children.forEach((child) => {
			traverse(
				child as { children: {}[]; data: { name: string; path: string; is_dir: boolean } },
				i + 1
			);
		});
		count++;
	};

	let testData: FileData[] = [];
	for (let i = 0; i < 100; i++) {
		testData.push({
			name: `test${i}`,
			type: FileType.DIRECTORY,
			size: i * i * 1000,
			lastModifiedDate: Date.now() + i * 1000,
			createdDate: Date.now() + i * 1000,
			owner: 'owner',
			permissions: 'read'
		} as FileData);
	}
</script>

<div class="container">
	<div class="topbar-container">
		<Topbar />
	</div>
	<div class="content-container">
		<div class="sidebar-container" style="width: {$settings.sidebar.width}em">
			<Sidebar />
		</div>
		<div class="file-list-container" style="width: calc(100% - {$settings.sidebar.width}em)">
			<FileList files={testData} />
		</div>
	</div>
	<div class="bottombar-container">
		<Bottombar />
	</div>
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
		border-right: 1px solid var(--color-secondary-dark);
	}

	.file-list-container {
		height: 100%;
	}

	.bottombar-container {
		height: 2em;
		width: 100%;
		border-top: 1px solid var(--color-secondary-dark);
	}
</style>
