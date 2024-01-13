<script lang="ts">
	import { invoke } from '@tauri-apps/api/tauri';
	import FileList from '$lib/components/FileList.svelte';
	import { FileType } from '$lib/types';
	import type { FileData } from '$lib/types';

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
	/*
  <input type="text" bind:value={input}>
  <div class="bc">
    <button on:click={search}>Search</button>
    <button on:click={addLocation}>Add Location</button>
    <button on:click={removeLocation}>Remove Location</button>
    <button on:click={reindexLocation}>Reindex Location</button>
    <button on:click={getTree}>Get Tree</button>
  </div>
  <Svg src="/svgs/folder/folder_default.svg" width={100} height={100} />
	{#each testList as test}
		<div>{test}</div>
	{/each}
  <div>{count}</div>
  */

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
	<FileList files={testData} />
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

	div {
		margin: 0px;
		padding: 0px;
		color: white;
		white-space: pre;
	}

	button {
		margin: 0px;
		padding: 0px;
		color: white;
		background-color: black;
		border: 1px solid white;
		width: 150px;
	}

	.bc {
		display: flex;
		flex-direction: row;
		align-items: center;
		justify-content: space-evenly;
	}
</style>
