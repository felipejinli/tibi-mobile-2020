import React from 'react';
import {View, Image, TouchableOpacity, Text} from 'react-native';
import PropTypes from 'prop-types';

import {fonts, sizes, colors, icons} from '../../../constants';

const CircularCard = ({item}) => {
  const cardWidth = sizes.width * 0.36;
  return (
    <View
      style={{
        // backgroundColor: 'white',
        alignItems: 'center',
        justifyContent: 'center',
        marginHorizontal: sizes.s,
      }}>
      <Image
        source={item.img}
        resizeMode="cover"
        style={{
          width: cardWidth,
          height: cardWidth,
          borderRadius: cardWidth / 2,
        }}
      />

      <Text
        style={{
          ...fonts.h5C,
          //   backgroundColor: 'burlywood',
          marginTop: sizes.s,
          width: cardWidth * 0.9,
          //   textAlign: 'center',
        }}
        numberOfLines={1}>
        {item.name}
      </Text>
    </View>
  );
};

CircularCard.propTypes = {
  item: PropTypes.shape({
    id: PropTypes.number.isRequired,
    favorite: PropTypes.bool,
    img: PropTypes.any.isRequired,
    name: PropTypes.string.isRequired,
  }),
};

export default CircularCard;
