import { FileType, type IndexLocation, type Pinned, type Settings, type WindowSettings } from '$lib/types';
import { invoke } from '@tauri-apps/api';
import { settings, windowSettings } from '$lib/stores';
import { get } from 'svelte/store';

let skipSave = false;

export const saveSettings = async () => {
	let settingsData = get(settings);
	if (!settingsData.loaded) return true;

	if (skipSave) {
		skipSave = false;
		return true;
	}

	const res = await invoke('save_settings', { settings: JSON.stringify(settingsData) }).catch(
		(err: Error) => {
			console.log(err);
		}
	);
	return res;
};

export const getSettings = async () => {
	const res = await invoke('get_settings').catch((err: Error) => {
		console.log(err);
	});

	if (typeof res !== 'string') return false;
	if (res === '') {
		get(settings).loaded = true;
		setDefaultSettings();
		return false;
	}

	let settingsData = JSON.parse(res) as Settings;

	skipSave = true;
	settingsData.loaded = true;
	settings.set(settingsData);

	return true;
};

export const saveWindowSettings = async () => {
	if (!get(settings).loaded) return true;

	const res = await invoke('save_window_settings', {
		windowSettings: JSON.stringify(get(windowSettings))
	}).catch((err: Error) => {
		console.log(err);
	});
	return res;
}

export const getWindowSettings = async () => {
	const res = await invoke('get_window_settings').catch((err: Error) => {
		console.log(err);
	});

	if (typeof res !== 'string') return false;
	if (res === '') {
		return false;
	}

	let settingsData = JSON.parse(res) as WindowSettings;

	windowSettings.set(settingsData);

	return true;
};

export const getUserName = async () => {
	const res = await invoke('get_username').catch((err: Error) => {
		console.log(err);
	});
	return res;
};

const setDefaultSettings = async () => {
	let username = await getUserName();
	get(settings).sidebar.pinnedPaths = [
		{
			name: 'Desktop',
			path: `C:/Users/${username}/Desktop`,
			type: FileType.DIRECTORY
		} as Pinned,
		{
			name: 'Documents',
			path: `C:/Users/${username}/Documents`,
			type: FileType.DIRECTORY
		} as Pinned,
		{
			name: 'Downloads',
			path: `C:/Users/${username}/Downloads`,
			type: FileType.DIRECTORY
		} as Pinned,
		{
			name: 'Music',
			path: `C:/Users/${username}/Music`,
			type: FileType.DIRECTORY
		} as IndexLocation,
		{
			name: 'Pictures',
			path: `C:/Users/${username}/Pictures`,
			type: FileType.DIRECTORY
		} as Pinned,
		{
			name: 'Videos',
			path: `C:/Users/${username}/Videos`,
			type: FileType.DIRECTORY
		} as Pinned
	];
	saveSettings();
};
