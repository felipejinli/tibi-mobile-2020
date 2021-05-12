import React, {useState, useEffect} from 'react';
import {Pressable, ImageBackground, View, Text} from 'react-native';

import {fonts, colors, sizes} from '../../constants';

const InterestCard = ({index, label, image, style, onChange}) => {
  const [selected, setSelected] = useState(false);

  let cardHeight = sizes.height / 10;
  let cardWidth = sizes.width / 2.35;

  useEffect(() => {
    onChange(index, selected);
    // console.log(index, label, selected);
  }, [selected]);

  return (
    <Pressable
      style={[
        {
          height: cardHeight,
          width: cardWidth,
          justifyContent: 'center',
          borderRadius: 5,
          borderWidth: 1,
          // borderColor: 'white',
          backgroundColor: 'black',
        },
        style,
      ]}
      onPress={() => {
        setSelected(!selected);
      }}>
      {selected && (
        <View
          style={{
            flex: 1,
            position: 'absolute',
            left: 0,
            top: 0,
            opacity: 0.63,
            backgroundColor: colors.primary,
            borderRadius: 5,
            height: cardHeight,
            width: cardWidth,
            zIndex: 10,
          }}
        />
      )}

      <ImageBackground
        style={{
          position: 'absolute',
          width: cardWidth * 0.62,
          alignSelf: 'flex-end',
          height: '100%',
          right: 0,
        }}
        imageStyle={{
          borderTopRightRadius: 5,
          borderBottomRightRadius: 5,
        }}
        source={{
          uri: image,
        }}
        resizeMode="cover">
        <View style={{flex: 1, backgroundColor: 'rgba(0,0,0,0.2)'}} />
      </ImageBackground>
      <View
        style={{
          height: '80%',
          marginLeft: 10,
          marginRight: 16,
          justifyContent: 'space-between',
        }}>
        <Text style={fonts.h1} numberOfLines={2}>
          {label}
        </Text>
      </View>
    </Pressable>
  );
};

export default InterestCard;
