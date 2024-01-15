import { IconTheme } from '$lib/types';
import type { SvgColor } from '$lib/types';

export const getFileThemeColors = (iconTheme: IconTheme) => {
	switch (iconTheme) {
		case IconTheme.BLUE:
      return [
				{ key: 'color_0', color: '17253D' },
				{ key: 'color_1', color: '192841' },
				{ key: 'color_2', color: '1c2e4a' },
				{ key: 'color_3', color: '203354' },
				{ key: 'color_4', color: '2E4B79' },
				{ key: 'color_5', color: '2C4672' },
				{ key: 'color_6', color: '28406A' },
				{ key: 'color_7', color: '19253A' },
				{ key: 'color_8', color: '1C2A41' },
				{ key: 'color_9', color: '1F2E47' }
			] as SvgColor[];
		case IconTheme.GREEN:
			return [] as SvgColor[];
		case IconTheme.RED:
			return [] as SvgColor[];
		case IconTheme.YELLOW:
			return [] as SvgColor[];
		case IconTheme.PURPLE:
			return [] as SvgColor[];
	}
};
