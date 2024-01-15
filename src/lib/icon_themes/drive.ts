import { IconTheme } from '$lib/types';
import type { SvgColor } from '$lib/types';
import { getPrimaryColor } from '$lib/utils/icon_resolver';

export const getDriveThemeColors = (iconTheme: IconTheme) => {
	return [{ key: 'color_0', color: getPrimaryColor(iconTheme) }] as SvgColor[];
};
