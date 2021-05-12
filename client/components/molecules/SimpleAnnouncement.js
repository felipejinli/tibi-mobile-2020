import React from 'react';
import {StyleSheet, Text, ImageBackground, View} from 'react-native';
import StyledText from 'react-native-styled-text';

import {colors, fonts, sizes} from '../../constants';
import {handleImageProp} from '../../utility/helpers';

const SimpleAnnouncement = ({item}) => {
  return (
    <ImageBackground
      style={{height: '100%', width: '100%'}}
      source={handleImageProp(item.announcementImage)}
      resizeMode="cover">
      <View style={{flex: 1, backgroundColor: 'rgba(0,0,0,0.2)'}}>
        <View
          style={{
            marginHorizontal: 24,
            marginVertical: 40,
          }}>
          <StyledText style={[fonts.display4]} numberOfLines={2}>
            {item.announcementSubtitle}
          </StyledText>
          <StyledText
            numberOfLines={3}
            style={[{paddingTop: sizes.s}, fonts.display1]}
            textStyles={StyleSheet.create({h: {color: colors.primary}})}>
            {item.announcementTitle}
          </StyledText>
        </View>
      </View>
    </ImageBackground>
  );
};

export default SimpleAnnouncement;
