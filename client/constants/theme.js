import {StyleSheet, Dimensions} from 'react-native';

const {width, height} = Dimensions.get('window');

export const colors = {
  primary: '#ff4800',
  white: '#ffffff',
  gray: '#a3a4a6',
  secondary: '#912900',
  black: '#000000',
  blueGrey: '#8e8e93',
  primary50: 'rgba(255, 72, 0, 0.5)',
};

export const fonts = StyleSheet.create({
  largeTitle: {
    fontFamily: 'TitlingGothicFBWide-Standard',
    fontSize: 40,
    fontWeight: 'normal',
    fontStyle: 'normal',
    lineHeight: 44,
    letterSpacing: 0,
    color: colors.white,
  },
  display1: {
    fontFamily: 'TitlingGothicFB Wide',
    fontSize: 24,
    fontWeight: 'bold',
    fontStyle: 'normal',
    lineHeight: 24,
    letterSpacing: 0,
    color: colors.white,
  },
  h1: {
    fontFamily: 'TitlingGothicFBWide-Standard',
    fontSize: 24,
    fontWeight: 'normal',
    fontStyle: 'normal',
    lineHeight: 26,
    letterSpacing: 0.06,
    color: colors.white,
  },
  headerTitle: {
    fontFamily: 'TitlingGothicFB Normal',
    fontSize: 21,
    fontWeight: '500',
    fontStyle: 'normal',
    letterSpacing: 0.05,
    textAlign: 'center',
    color: colors.white,
  },
  h3C: {
    fontFamily: 'TitlingGothicFB Normal',
    fontSize: 16,
    fontWeight: '500',
    fontStyle: 'normal',
    lineHeight: 16,
    letterSpacing: 0,
    textAlign: 'center',
    color: colors.primary,
  },
  h4: {
    fontFamily: 'TitlingGothicFBWide-Standard',
    fontSize: 14,
    fontWeight: 'normal',
    fontStyle: 'normal',
    lineHeight: 16,
    letterSpacing: 0,
    color: colors.white,
  },
  display4: {
    fontFamily: 'TitlingGothicFB Wide',
    fontSize: 13,
    fontWeight: '300',
    fontStyle: 'normal',
    letterSpacing: -0.42,
    color: colors.white,
  },
  body2R: {
    fontFamily: 'TitlingGothicFB Normal',
    fontSize: 12,
    fontWeight: 'normal',
    fontStyle: 'normal',
    lineHeight: 16,
    letterSpacing: 0,
    textAlign: 'right',
    color: colors.white,
  },
  body3: {
    fontFamily: 'TitlingGothicFB Normal',
    fontSize: 11,
    fontWeight: 'normal',
    fontStyle: 'normal',
    letterSpacing: 0.17,
    color: colors.gray,
  },
  h5Rdetail: {
    fontFamily: 'TitlingGothicFBWide-Standard',
    fontSize: 11,
    fontWeight: 'normal',
    fontStyle: 'normal',
    letterSpacing: 0.14,
    textAlign: 'right',
    color: colors.primary,
  },
  h5: {
    fontFamily: 'TitlingGothicFBWide-Standard',
    fontSize: 11,
    fontWeight: 'normal',
    fontStyle: 'normal',
    lineHeight: 17,
    letterSpacing: -0.03,
    color: colors.white,
  },
  h5C: {
    fontFamily: 'TitlingGothicFBWide-Standard',
    fontSize: 11,
    fontWeight: 'normal',
    fontStyle: 'normal',
    lineHeight: 17,
    letterSpacing: -0.03,
    textAlign: 'center',
    color: colors.white,
  },
  body4: {
    fontFamily: 'TitlingGothicFB Normal',
    fontSize: 10,
    fontWeight: '500',
    fontStyle: 'normal',
    letterSpacing: 0.97,
    color: '#000001',
  },
});

export const sizes = {
  // margins and paddings
  xs: 4,
  s: 8,
  m: 12,
  l: 16,
  xl: 20,
  xxl: 24,

  //   fonts
  largeTitle: 40,
  h1: 24,
  h2: 21,
  h3: 16,
  h4: 14,
  h5: 11,

  body1: 14,
  body2: 12,
  body3: 11,
  body4: 10,

  width,
  height,
};
const AppTheme = {colors, fonts, sizes};
export default AppTheme;
// import {Dimensions} from 'react-native';

// const {width, height} = Dimensions.get('window');

// export const COLORS = {
//   // core colors
//   primary: '#ff4800',
//   secondary: '#912900',
//   black: '#1E1F20',
//   white: '#FFFFFF',
//   lightGray: '#eff2f5',
//   gray: '#BEC1D2',
// };

// export const FONTS = {
//   largeTitle: {
//     fontFamily: 'TitlingGothicFBWide-Standard',
//     fontSize: SIZES.largeTitle,
//     lineHeight: 44,
//   },
//   h1: {
//     fontFamily: 'TitlingGothicFBWide-Standard',
//     fontSize: SIZES.h1,
//     lineHeight: 26,
//     letterSpacing: 0.06,
//   },
//   h4: {
//     fontFamily: 'TitlingGothicFBWide-Standard',
//     fontSize: SIZES.h4,
//     lineHeight: 16,
//     letterSpacing: 0.06,
//   },
// };

// const AppTheme = {COLORS, SIZES, FONTS};

// export default AppTheme;
