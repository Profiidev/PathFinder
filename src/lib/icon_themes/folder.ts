import { IconTheme } from '$lib/types';
import type { SvgColor } from '$lib/types';

export const getFolderThemeColors = (iconTheme: IconTheme) => {
	switch (iconTheme) {
		case IconTheme.BLUE:
			return [
				{ key: 'color_0', color: 'rgb(9, 132, 232)' },
				{ key: 'color_1', color: 'rgb(34, 96, 221)' },
				{ key: 'color_2', color: 'rgb(0, 119, 216)' },
				{ key: 'color_3', color: 'rgb(16, 111, 189)' },
				{ key: 'color_4', color: 'rgb(119, 193, 253)' },
				{ key: 'color_5', color: 'rgb(108, 187, 252)' },
				{ key: 'color_6', color: 'rgb(113, 191, 255)' },
				{ key: 'color_7', color: 'rgb(79, 176, 255)' },
				{ key: 'color_8', color: 'rgb(86, 179, 255)' },
				{ key: 'color_9', color: 'rgb(0, 140, 255)' },
				{ key: 'color_A', color: 'rgb(43, 160, 255)' },
				{ key: 'color_B', color: 'rgb(67, 169, 253)' },
				{ key: 'color_C', color: 'rgb(89, 180, 255)' },
				{ key: 'color_D', color: 'rgb(119, 194, 255)' },
			] as SvgColor[];
		case IconTheme.GREEN:
			return [
				{ key: 'color_0', color: '#066328' },
				{ key: 'color_1', color: '#0B6229' },
				{ key: 'color_2', color: '#085925' },
				{ key: 'color_3', color: '#0A5022' },
				{ key: 'color_4', color: '#188743' },
				{ key: 'color_5', color: '#198643' },
				{ key: 'color_6', color: '#15843D' },
				{ key: 'color_7', color: '#0C8137' },
				{ key: 'color_8', color: '#0A7932' },
				{ key: 'color_9', color: '#086127' },
				{ key: 'color_A', color: '#087730' },
				{ key: 'color_B', color: '#087730' },
				{ key: 'color_C', color: '#0A8035' },
				{ key: 'color_D', color: '#13823B' }
			] as SvgColor[];
		case IconTheme.RED:
			return [
				{ key: 'color_0', color: '#7B0400' },
				{ key: 'color_1', color: '#7B0200' },
				{ key: 'color_2', color: '#710200' },
				{ key: 'color_3', color: '#680300' },
				{ key: 'color_4', color: '#AC0900' },
				{ key: 'color_5', color: '#B00A01' },
				{ key: 'color_6', color: '#A70600' },
				{ key: 'color_7', color: '#9D0500' },
				{ key: 'color_8', color: '#8F0500' },
				{ key: 'color_9', color: '#7B0200' },
				{ key: 'color_A', color: '#8F0500' },
				{ key: 'color_B', color: '#8F0500' },
				{ key: 'color_C', color: '#990500' },
				{ key: 'color_D', color: '#A20500' }
			] as SvgColor[];
		case IconTheme.YELLOW:
			return [
				{ key: 'color_0', color: '#FAC017' },
				{ key: 'color_1', color: '#E1AB2D' },
				{ key: 'color_2', color: '#E3A917' },
				{ key: 'color_3', color: '#D79C1E' },
				{ key: 'color_4', color: '#FFEFB2' },
				{ key: 'color_5', color: '#FFEDAD' },
				{ key: 'color_6', color: '#FFE99F' },
				{ key: 'color_7', color: '#FEE289' },
				{ key: 'color_8', color: '#FED86B' },
				{ key: 'color_9', color: '#FEC832' },
				{ key: 'color_A', color: '#FCD667' },
				{ key: 'color_B', color: '#FDDA75' },
				{ key: 'color_C', color: '#FEE496' },
				{ key: 'color_D', color: '#FFE8A2' }
			] as SvgColor[];
		case IconTheme.PURPLE:
			return [
				{ key: 'color_0', color: '#4D00C1' },
				{ key: 'color_1', color: '#4701C0' },
				{ key: 'color_2', color: '#4401AF' },
				{ key: 'color_3', color: '#38019E' },
				{ key: 'color_4', color: '#7E1CFF' },
				{ key: 'color_5', color: '#7A1CFF' },
				{ key: 'color_6', color: '#7213FF' },
				{ key: 'color_7', color: '#6A06FF' },
				{ key: 'color_8', color: '#5C00F0' },
				{ key: 'color_9', color: '#4A00C1' },
				{ key: 'color_A', color: '#5D00E7' },
				{ key: 'color_B', color: '#5E00EC' },
				{ key: 'color_C', color: '#6702FF' },
				{ key: 'color_D', color: '#6F0FFF' }
			] as SvgColor[];
	}
};
