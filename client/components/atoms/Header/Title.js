import React from 'react';
import {Text} from 'react-native';
import {colors, fonts} from '../../../constants';

function Title() {
  return (
    <Text style={{...fonts.headerTitle}}>
      Explore<Text style={{color: colors.primary}}>UCL</Text>
    </Text>
  );
}

export default Title;
