import React from 'react';
import {TouchableOpacity, ImageBackground, View, Text} from 'react-native';

import {images, fonts, colors, sizes} from '../../constants';

const EventListCard = ({item, style}) => {
  return (
    <TouchableOpacity
      style={[
        {
          height: sizes.height * 0.2,
          width: sizes.width,
          //   backgroundColor: 'burlywood',
          justifyContent: 'center',
        },
        style,
      ]}
      onPress={() => console.log('FJL eventListCard pressed')}>
      <ImageBackground
        style={{
          position: 'absolute',
          width: sizes.width * 0.65,
          alignSelf: 'flex-end',
          height: '100%',
          backgroundColor: 'red',
          right: 0,
        }}
        source={{
          uri:
            'https://images.unsplash.com/photo-1599945439730-fc0342517915?ixlib=rb-1.2.1&q=80&fm=jpg&crop=entropy&cs=tinysrgb&w=1080&fit=max&ixid=eyJhcHBfaWQiOjF9',
        }}
        resizeMode="cover">
        <View style={{flex: 1, backgroundColor: 'rgba(0,0,0,0.2)'}} />
        {/* <View style={{flex: 1, backgroundColor: 'yellow'}} /> */}
      </ImageBackground>
      <View
        style={{
          marginLeft: 24,
          //   marginVertical: 18,
          height: '80%',
          marginRight: 16,
          justifyContent: 'space-between',
          //   backgroundColor: 'yellow',
        }}>
        <Text
          style={[fonts.h1, {paddingRight: sizes.width * 0.2}]}
          numberOfLines={2}>
          EFS Christmas Ball
        </Text>
        <View
          style={{
            flexDirection: 'row',
            justifyContent: 'space-between',
            alignItems: 'center',
          }}>
          <Text style={[fonts.h5Rdetail, {textAlign: 'left'}]}>5pm</Text>
          <Text style={[fonts.h5Rdetail, {maxWidth: '50%'}]} numberOfLines={1}>
            08 Dec
          </Text>
        </View>
      </View>
    </TouchableOpacity>
  );
};

export default EventListCard;
