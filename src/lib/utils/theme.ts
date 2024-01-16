import { Theme } from "$lib/types";

export const getPrimaryColor = (colorTheme: Theme) => {
  switch (colorTheme) {
    case Theme.LIGHT:
      return '#F6F6F6';
    case Theme.DARK:
      return '#1C1D26';
  }
}

export const getPrimaryDarkColor = (colorTheme: Theme) => {
  switch (colorTheme) {
    case Theme.LIGHT:
      return '#ececec';
    case Theme.DARK:
      return '#0D0D16';
  }
}

export const getSecondaryColor = (colorTheme: Theme) => {
  switch (colorTheme) {
    case Theme.LIGHT:
      return '#dadada';
    case Theme.DARK:
      return '#34343D';
  }
}

export const getSecondaryDarkColor = (colorTheme: Theme) => {
  switch (colorTheme) {
    case Theme.LIGHT:
      return '#dbdbdb';
    case Theme.DARK:
      return '#23232C';
  }
}

export const getSecondaryLightColor = (colorTheme: Theme) => {
  switch (colorTheme) {
    case Theme.LIGHT:
      return '#292929';
    case Theme.DARK:
      return '#9292A0';
  }
}

export const getTextColor = (colorTheme: Theme) => {
  switch (colorTheme) {
    case Theme.LIGHT:
      return '#1d1d1d';
    case Theme.DARK:
      return '#FFFFFF';
  }
}