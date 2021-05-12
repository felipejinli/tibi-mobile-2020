import React, {Component} from 'react';
import {ImageBackground, View, Text, Image} from 'react-native';
import Swiper from 'react-native-swiper';

import {colors, fonts, sizes} from '../../constants';
import SimpleAnnouncement from '../molecules/SimpleAnnouncement';

const AnnouncementSwiper = ({data, style}) => {
  return (
    <Swiper
      style={[
        {
          height: sizes.height * 0.36,
          //   width: sizes.width,
          //   justifyContent: 'center',
          //   alignItems: 'center',
        },
        style,
      ]}
      autoplay
      autoplayTimeout={8}
      showsButtons={false}
      dot={
        <View
          style={{
            backgroundColor: 'rgba(255, 255, 255, 0.1)',
            width: 16,
            height: 3,
            marginHorizontal: sizes.xs,
            borderRadius: 1.5,
            bottom: -sizes.m,
          }}
        />
      }
      activeDot={
        <View
          style={{
            backgroundColor: colors.primary,
            width: 24,
            height: 3,
            marginHorizontal: sizes.xs,
            borderRadius: 1.5,
            bottom: -sizes.m,
          }}
        />
      }>
      {data.map((item, index) => {
        return (
          <React.Fragment key={item._id}>
            <SimpleAnnouncement item={item} />
          </React.Fragment>
        );
      })}
    </Swiper>
  );
};

export default AnnouncementSwiper;
