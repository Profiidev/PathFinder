import { FileType, IconTheme } from "$lib/types";

export const getIconPath = (fileType: FileType, iconTheme: IconTheme) => {
  switch (fileType) {
		case FileType.FILE:
			return `/svgs/file/file_${iconTheme}.svg`;
		case FileType.DIRECTORY:
			return `/svgs/folder/folder_${iconTheme}.svg`;
		default:
			return `/svgs/file/file_${iconTheme}.svg`;
	}
}