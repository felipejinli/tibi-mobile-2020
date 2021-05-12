# Tibi Global Mobile App

## Set Up (Starting from client)

Install Dependencies

1. `yarn install`
2. `cd ios`
3. `pod install`

Run on Simulator

1. `npx react-native start`
2. `npx react-native run-ios`

Run on iPhone

1. `yarn global add ios-deploy`
2. `npx react-native start`
3. `npx react-native run-ios --device "Device Name"`

## Troubleshooting

### Known Errors

- Font file naming
  - iOS and Android read custom font file names differently.
  - font styles are imported straight from our Zeplin styleguide and this causes the fontFamily style property to incorrectly call 'TitlingGothicFBNormal' for a font with say filename TitlingGothicFBNormalRegular.otf
  - The root error is that iOS uses the full name property of font files, which is like metadata in which case fontFamily: 'TitlingGothicFB Normal' is correct.

---

## Data and Backend Connection

### Data Schemas

- Image component from React Native has source props taking `source={uri: 'https://example.com/image.png}` if we want to pass a live link or it takes a direct `source: require('./path/to/image.png')`
