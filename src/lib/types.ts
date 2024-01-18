export type FileData = {
	name: string;
	type: FileType;
	path: string;
	size: number;
	lastModifiedDate: number;
	createdDate: number;
	permissions: string;
	hidden: boolean;
};

export enum FileType {
	FILE = 'file',
	DIRECTORY = 'directory',
	DRIVE = 'drive'
}

export enum Theme {
	DARK = 'dark',
	LIGHT = 'light'
}

export enum IconTheme {
	BLUE = 'blue',
	GREEN = 'green',
	RED = 'red',
	YELLOW = 'yellow',
	PURPLE = 'purple'
}

export type Settings = {
	fileList: FileListSettings;
	appearance: AppearanceSettings;
	sidebar: SidebarSettings;
	currentPath: string;
	loaded: boolean;
};

export type AppearanceSettings = {
	zoom: number;
	theme: Theme;
	iconTheme: IconTheme;
	viewType: ViewType;
};

export type FileListSettings = {
	showHiddenFiles: boolean;
	showFileExtensions: boolean;
	fileListHeaders: FileListHeader[];
	sortType: SortType;
	sortAscending: boolean;
};

export type FileListHeader = {
	name: string;
	sortType: SortType;
	width: number;
	active: boolean;
	align_text_right?: boolean;
};

export enum SortType {
	NAME = 'name',
	SIZE = 'size',
	CREATED_DATE = 'created_date',
	MODIFIED_DATE = 'modified_date',
	PERMISSIONS = 'permissions',
	TYPE = 'type'
}

export type FileSelectionData = {
	files: string[];
	lastSelectedIndex: number;
};

export type SidebarSettings = {
	pinnedPaths: Pinned[];
	devices: Device[];
	tags: Tag[];
	width: number;
	expanded: boolean[];
};

export type Tag = {
	name: string;
	color: string;
};

export type Device = {
	name: string;
	path: string;
};

export type Pinned = {
	name: string;
	path: string;
	type: FileType;
};

export type IndexLocation = {
	name: string;
	path: string;
	type: FileType;
};

export enum ViewType {
	LIST = 'list',
	GRID = 'grid'
}

export type SvgColor = {
	key: string;
	color: string;
};

export type SvgData = {
	path: string;
	colors: SvgColor[];
};

export type SvgInfo = {
	data: SvgData;
	width: number;
	height: number;
};

export type LocationData = {
	name: string;
	path: string;
	type: FileType;
};

export type PathHistory = {
	paths: string[];
	currentIndex: number;
	historyUpdated: boolean;
};