import type { Settings } from '$lib/types';
import { invoke } from '@tauri-apps/api';
import { settings } from '$lib/stores';
import { get } from 'svelte/store';

export const saveSettings = async () => {
	let settingsData = get(settings);
	if (!settingsData.loaded) return true;
	
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
		saveSettings();
		return false;
	}

	let settingsData = JSON.parse(res) as Settings;
	
	settingsData.loaded = true;
	settings.set(settingsData);

	return true;
};
