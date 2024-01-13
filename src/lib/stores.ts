import { writable } from 'svelte/store';
import type { FileSelectionData } from './types';
import { defaultSettings } from './utils/constants';

export const settings = writable(defaultSettings);

export const selectedFiles = writable({
	files: [] as string[],
	lastSelectedIndex: -1
} as FileSelectionData);

export const pressedKeys = writable([] as string[]);
