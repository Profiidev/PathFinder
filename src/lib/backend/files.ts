import { FileType, type FileData } from '$lib/types';
import { invoke } from '@tauri-apps/api';
import { loadedFiles, settings } from '$lib/stores';
import { get } from 'svelte/store';

export const loadFiles = async () => {
	const res = await invoke('get_files', { location: get(settings).currentPath }).catch((err) => {
		console.log(err);
	});
  if (!(res instanceof Array)) return false;
  
  let files = res.map(file => {
    return {
			name: file.file.name as string,
			path: file.path as string,
			type: file.file.is_dir ? FileType.DIRECTORY : FileType.FILE,
			size: file.file.size as number,
			lastModifiedDate: file.last_modified_date as number,
			createdDate: file.created_date as number,
      permissions: file.permissions as string,
      hidden : file.hidden as boolean
		} as FileData;
  });
  
  loadedFiles.set(files);
};
