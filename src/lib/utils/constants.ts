import { IconTheme, SortType, Theme } from "$lib/types";
import type { AppearanceSettings, FileListHeader, FileListSettings, Settings } from "$lib/types";

export const fileListHeaders = [
  { name: 'Name', sortType: SortType.NAME, width: 12.6, active: true } as FileListHeader,
  { name: 'Created', sortType: SortType.CREATED_DATE, width: 12, active: true } as FileListHeader,
  { name: 'Modified', sortType: SortType.MODIFIED_DATE, width: 12, active: true } as FileListHeader,
  { name: 'Owner', sortType: SortType.OWNER, width: 6.3, active: true } as FileListHeader,
	{ name: 'Permissions', sortType: SortType.PERMISSIONS, width: 8.6, active: true } as FileListHeader,
	{ name: 'Type', sortType: SortType.TYPE, width: 9, active: true } as FileListHeader,
  { name: 'Size', sortType: SortType.SIZE, width: 6.3, active: true, align_text_right: true } as FileListHeader,
];

export const defaultSettings = {
	fileList: {
		showHiddenFiles: true,
		showFileExtensions: true,
    fileListHeaders: fileListHeaders,
		sortType: SortType.NAME,
		sortAscending: true
	} as FileListSettings,
	appearance: {
		zoom: .5,
		theme: Theme.DARK,
		iconTheme: IconTheme.BLUE
	} as AppearanceSettings
} as Settings;

export const minZoom = 0.1;
export const maxZoom = 2;
export const zoomStep = 0.1;

export const fileListHeaderMinWidth = 5;