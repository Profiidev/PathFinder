export type FileData = {
	name: string;
	type: FileType;
	size: number;
	lastModifiedDate: number;
	createdDate: number;
	owner: string;
	permissions: string;
};

export enum FileType {
	FILE = 'file',
	DIRECTORY = 'directory'
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
};

export type AppearanceSettings = {
	zoom: number;
	theme: Theme;
	iconTheme: IconTheme;
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
};

export enum SortType {
	NAME = 'name',
	SIZE = 'size',
	CREATED_DATE = 'created_date',
	MODIFIED_DATE = 'modified_date',
	OWNER = 'owner',
	PERMISSIONS = 'permissions'
}

export type FileSelectionData = {
	files: string[];
	lastSelectedIndex: number;
};
