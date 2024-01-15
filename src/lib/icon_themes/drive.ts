import { IconTheme } from '$lib/types';
import type { SvgColor } from '$lib/types';

export const getDriveThemeColors = (iconTheme: IconTheme) => {
	switch (iconTheme) {
		case IconTheme.BLUE:
			return [{ key: 'color_0', color: '1d98ff' }] as SvgColor[];
		case IconTheme.GREEN:
			return [{ key: 'color_0', color: '#4caf50' }] as SvgColor[];
		case IconTheme.RED:
			return [{ key: 'color_0', color: '#f44336' }] as SvgColor[];
		case IconTheme.YELLOW:
			return [{ key: 'color_0', color: '#FAC017' }] as SvgColor[];
		case IconTheme.PURPLE:
			return [{ key: 'color_0', color: '#9c27b0' }] as SvgColor[];
	}
};
