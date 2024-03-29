import { FileType, IconTheme, SortType, Theme, ViewType } from '$lib/types';
import type {
	AppearanceSettings,
	FileListHeader,
	FileListSettings,
	Settings,
	SidebarSettings,
	Tag,
	Pinned,
	WindowSettings
} from '$lib/types';

export const fileListHeaders = [
	{ name: 'Name', sortType: SortType.NAME, width: 12.6, active: true } as FileListHeader,
	{ name: 'Created', sortType: SortType.CREATED_DATE, width: 12, active: true } as FileListHeader,
	{ name: 'Modified', sortType: SortType.MODIFIED_DATE, width: 12, active: true } as FileListHeader,
	{
		name: 'Permissions',
		sortType: SortType.PERMISSIONS,
		width: 8.6,
		active: true
	} as FileListHeader,
	{ name: 'Type', sortType: SortType.TYPE, width: 9, active: true } as FileListHeader,
	{
		name: 'Size',
		sortType: SortType.SIZE,
		width: 6.3,
		active: true,
		align_text_right: true
	} as FileListHeader
];

export const defaultSettings = {
	fileList: {
		showHiddenFiles: true,
		showFileExtensions: true,
		fileListHeaders: fileListHeaders,
	} as FileListSettings,
	appearance: {
		theme: Theme.DARK,
		iconTheme: IconTheme.BLUE,
	} as AppearanceSettings,
	sidebar: {
		pinnedPaths: [] as Pinned[],
		devices: [],
		tags: [
			{ name: 'Important', color: '#ff0000' } as Tag,
			{ name: 'Work', color: '#00ff00' } as Tag,
			{ name: 'Personal', color: '#0000ff' } as Tag
		] as Tag[],
		width: 10,
	} as SidebarSettings,
	loaded: false,
} as Settings;

export const defaultWindowSettings = {
	currentPath: 'C:/',
	useRegex: false,
	caseSensitive: false,
	sortType: SortType.NAME,
	sortAscending: true,
	zoom: 0.5,
	viewType: ViewType.LIST,
	expanded: [true, true, true, true],
} as WindowSettings;

export const minZoom = 0.1;
export const maxZoom = 2;
export const zoomStep = 0.1;

export const fileListHeaderMinWidth = 5;
export const sidebarMinWidth = 7.5;
