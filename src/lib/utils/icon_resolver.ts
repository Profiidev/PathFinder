import { getFileThemeColors } from "$lib/icon_themes/file";
import { getFolderThemeColors } from "$lib/icon_themes/folder";
import { getDriveThemeColors } from "$lib/icon_themes/drive";
import { FileType, IconTheme } from "$lib/types";
import type { SvgColor, SvgData } from "$lib/types";

export const getIconData = (fileType: FileType, iconTheme: IconTheme) => {
	let path = "";
  switch (fileType) {
		case FileType.FILE:
			path = `/svgs/file/file.svg`;
			break;
		case FileType.DIRECTORY:
			path = `/svgs/folder/folder.svg`;
			break;
		case FileType.DRIVE:
			path = `/svgs/drive/drive.svg`;
			break;
		default:
			path = `/svgs/file/file.svg`;
			break;
	}

	let colors = [] as SvgColor[];
	switch (fileType) {
		case FileType.FILE:
			colors = getFileThemeColors(iconTheme);
			break;
		case FileType.DIRECTORY:
			colors = getFolderThemeColors(iconTheme);
			break;
		case FileType.DRIVE:
			colors = getDriveThemeColors(iconTheme);
			break;
		default:
			colors = getFileThemeColors(iconTheme);
			break;
	}

	return { path, colors } as SvgData;
}

export const getPrimaryColor = (iconTheme: IconTheme) => {
	switch (iconTheme) {
		case IconTheme.BLUE:
			return '1c1d26';
		case IconTheme.GREEN:
			return '4caf50';
		case IconTheme.RED:
			return 'f44336';
		case IconTheme.YELLOW:
			return 'FAC017';
		case IconTheme.PURPLE:
			return '9c27b0';
	}
}

export const getSecondaryColor = (iconTheme: IconTheme) => {
	switch (iconTheme) {
		case IconTheme.BLUE:
			return '002ab6';
		case IconTheme.GREEN:
			return '4caf50';
		case IconTheme.RED:
			return 'f44336';
		case IconTheme.YELLOW:
			return 'FAC017';
		case IconTheme.PURPLE:
			return '9c27b0';
	}
}