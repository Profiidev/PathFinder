import { writable } from 'svelte/store';
import type { FileSelectionData, FileData, LocationData, PathHistory } from './types';
import { defaultSettings } from './utils/constants';

export const settings = writable(defaultSettings);

export const selectedFiles = writable({
	files: [] as string[],
	lastSelectedIndex: -1
} as FileSelectionData);

export const loadedFiles = writable([] as FileData[]);

export const pressedKeys = writable([] as string[]);

export const settingsEnabled = writable(false);

export const locations = writable([] as LocationData[]);

export const pathHistory = writable({paths: [] as string[], currentIndex: -1, historyUpdated: false} as PathHistory);