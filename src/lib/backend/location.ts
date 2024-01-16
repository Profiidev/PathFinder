import { FileType, type LocationData } from '$lib/types';
import { invoke } from '@tauri-apps/api/tauri';
import { locations } from '$lib/stores';

export const addLocation = async (location: string) => {
	const res = await invoke('add_location', { location: location }).catch((err: Error) => {
		console.log(err);
	});
	return res;
};

export const removeLocation = async (location: string) => {
	const res = await invoke('remove_location', { location: location }).catch((err: Error) => {
		console.log(err);
	});
	return res;
};

export const reindexLocation = async (location: string) => {
	const res = await invoke('reindex_location', { location: location }).catch((err: Error) => {
		console.log(err);
	});
	return res;
};

export const updateLocations = async () => {
	const res = await invoke('get_locations').catch((err: Error) => {
		console.log(err);
	});
	if (!(res instanceof Array)) return [];

	let locationsData: LocationData[] = [];

	res.forEach((location: string) => {
		let first = location.split('/').filter(l => l !== '').pop() || '';

		locationsData.push({
			name: first.includes(':') ? `Volume (${first})` : first,
			path: location,
			type: first.includes(':') ? FileType.DRIVE : FileType.DIRECTORY
		});
	});
  
	locations.set(locationsData);
};
