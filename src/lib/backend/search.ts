import { invoke } from "@tauri-apps/api";
import { settings } from "$lib/stores";
import { get } from "svelte/store";
import { FileType, type FileData } from "$lib/types";
import { loadedFiles } from "$lib/stores";

export const search = async (search: string) => {
	const res = await invoke('search_partial', {
		name: search,
		path: get(settings).currentPath,
		useRegex: get(settings).useRegex,
    indexStart: 0,
		indexEnd: 10000,
		searchId: Math.floor(Math.random() * 1000000000)
	}).catch((err) => {
		console.log(err);
  });
	if (!(res instanceof Array)) return false;
	if(search === '') return false;

	let files = res.map((file) => {
		return {
			name: file.file.name as string,
			path: file.path as string,
			type: file.file.is_dir ? FileType.DIRECTORY : FileType.FILE,
			size: file.file.size as number,
			lastModifiedDate: file.last_modified_date as number,
			createdDate: file.created_date as number,
			permissions: file.permissions as string,
			hidden: file.hidden as boolean
		} as FileData;
  });
	
  loadedFiles.set(files);
};