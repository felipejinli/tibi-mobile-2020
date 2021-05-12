import React from 'react';
import {Text, View, Image, TouchableOpacity, FlatList} from 'react-native';

import {fonts, colors, sizes} from '../../constants';
import {SquareDetailCard, CircularCard, WideCard} from '../molecules/cards';

const HorizontalList = ({layout, data, style, title}) => {
  function renderCard(layout, item, index) {
    switch (layout) {
      case 'square-detail':
        // console.log('FJL entering squaredttailcard');
        return <SquareDetailCard item={item} />;
      case 'circular':
        // console.log('FJL entering circularCard');
        return <CircularCard item={item} />;
      case 'wide':
        // console.log('FJL entering wideCard');
        return <WideCard item={item} />;
      default:
        throw new Error(
          'HorizontalList.js entered default card layout render i.e. incorrect layout prop passed',
        );
    }
  }

  function switchSnapInterval(layout) {
    switch (layout) {
      case 'square-detail':
        return sizes.width * 0.44 + sizes.xs * 2;
      case 'circular':
        return sizes.width * 0.36 + sizes.s * 2;
      case 'wide':
        return sizes.width * 0.6 + sizes.xs * 2;
      default:
        throw new Error('FJL switchSnapInterval default unhandled case');
    }
  }

  return (
    <View style={style}>
      <View
        style={{
          flexDirection: 'row',
          justifyContent: 'space-between',
          alignItems: 'center',
          marginHorizontal: 16,
        }}>
        <Text style={{...fonts.h4, fontSize: 15}}>: {title}</Text>
        <TouchableOpacity onPress={() => console.log('see all onPressed')}>
          <Text style={{...fonts.body2r, color: colors.gray}}>See All</Text>
        </TouchableOpacity>
      </View>
      <FlatList
        horizontal
        showsHorizontalScrollIndicator={false}
        data={data}
        keyExtractor={(item) => item.id.toString()}
        renderItem={({item, index}) => renderCard(layout, item, index)}
        style={{
          //   backgroundColor: 'skyblue',
          marginTop: 14,
          marginLeft: 16,
        }}
        decelerationRate={'fast'}
        snapToInterval={switchSnapInterval(layout)}
        snapToAlignment={'start'}
      />
    </View>
  );
};

export default HorizontalList;
