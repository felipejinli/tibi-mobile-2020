import React, {useRef, useEffect, memo, useState, useMemo} from 'react';
import {
  Text,
  View,
  ActivityIndicator,
  Animated,
  RefreshControl,
} from 'react-native';
import {RecyclerListView, DataProvider, LayoutProvider} from 'recyclerlistview';

import {sizes} from '../../constants';
import EventListCard from '../molecules/EventListCard';

const Component = () => {
  const [newEvents, setNewEvents] = React.useState([
    {
      id: 80,
      date: '09/12/2020',
      details: {
        date: 'Sat 12 Sep',
        price: '£6.87',
        location: 'Cogilith',
      },
      favorite: true,
      img: {
        uri: 'http://dummyimage.com/213x212.png/dddddd/000000',
      },
      name: 'Washington',
    },
    {
      id: 328,
      date: '04/16/2020',
      details: {
        date: 'Thu 16 Apr',
        price: '£0.68',
        location: 'Lajo',
      },
      favorite: true,
      img: {
        uri: 'http://dummyimage.com/151x193.bmp/ff4444/ffffff',
      },
      name: 'Tubular Bells for Two',
    },
    {
      id: 162,
      date: '09/24/2019',
      details: {
        date: 'Tue 24 Sep',
        price: '£11.44',
        location: 'Jaxbean',
      },
      favorite: false,
      img: {
        uri: 'http://dummyimage.com/210x196.png/dddddd/000000',
      },
      name: 'MZAZA - The Birth and Death of Stars',
    },
    {
      id: 790,
      date: '07/07/2020',
      details: {
        date: 'Tue  7 Jul',
        price: '£5.54',
        location: 'Babblestorm',
      },
      favorite: false,
      img: {
        uri: 'http://dummyimage.com/150x166.png/cc0000/ffffff',
      },
      name: 'Washington',
    },
    {
      id: 833,
      date: '06/12/2020',
      details: {
        date: 'Fri 12 Jun',
        price: '£11.62',
        location: 'Geba',
      },
      favorite: false,
      img: {
        uri: 'http://dummyimage.com/234x225.png/dddddd/000000',
      },
      name: 'Twilight Jazz by the River',
    },
    {
      id: 735,
      date: '04/14/2020',
      details: {
        date: 'Tue 14 Apr',
        price: '£1.12',
        location: 'Gevee',
      },
      favorite: false,
      img: {
        uri: 'http://dummyimage.com/170x222.png/5fa2dd/ffffff',
      },
      name: 'The Sound Society',
    },
    {
      id: 889,
      date: '08/16/2020',
      details: {
        date: 'Sun 16 Aug',
        price: '£11.15',
        location: 'Dabshots',
      },
      favorite: true,
      img: {
        uri: 'http://dummyimage.com/152x178.png/dddddd/000000',
      },
      name: 'Washington',
    },
    {
      id: 222,
      date: '05/12/2020',
      details: {
        date: 'Tue 12 May',
        price: '£8.56',
        location: 'Realbridge',
      },
      favorite: false,
      img: {
        uri: 'http://dummyimage.com/166x182.png/5fa2dd/ffffff',
      },
      name: 'Blooms and Tunes - Cigany Weaver',
    },
    {
      id: 989,
      date: '09/16/2020',
      details: {
        date: 'Wed 16 Sep',
        price: '£4.99',
        location: 'Dynazzy',
      },
      favorite: false,
      img: {
        uri: 'http://dummyimage.com/244x135.bmp/dddddd/000000',
      },
      name: 'Percussion at Dusk',
    },
    {
      id: 646,
      date: '11/15/2019',
      details: {
        date: 'Fri 15 Nov',
        price: '£8.09',
        location: 'Quatz',
      },
      favorite: true,
      img: {
        uri: 'http://dummyimage.com/120x249.bmp/ff4444/ffffff',
      },
      name: "Lord Mayor's City Hall Concerts - Tilly Bebe: Women in Jazz",
    },
  ]);
  //   const scrollViewPos = useRef(new Animated.Value(0));
  const flatListRef = useRef();
  function renderNews(itemType, item, index) {
    const {id, favorite} = item;

    if (itemType === ViewTypes.BOOKED_EVENT) {
      return (
        <View style={{backgroundColor: 'red', width: sizes.width, height: 80}}>
          <Text>Header TYPE</Text>
        </View>
      );
    }

    return <EventListCard />;
  }

  const ViewTypes = {
    LIKED_EVENT: 0,
    BOOKED_EVENT: 1,
  };

  const dataProvider = useRef(new DataProvider((r1, r2) => r1 !== r2));

  const layoutProvider = useRef(
    new LayoutProvider(
      (index) => {
        if (index === 0) {
          return ViewTypes.BOOKED_EVENT;
        }
        return ViewTypes.LIKED_EVENT;
      },
      (itemType, dim) => {
        if (itemType === ViewTypes.BOOKED_EVENT) {
          dim.height = 100 + 120;
        } else {
          dim.height = sizes.height * 0.8;
        }
      },
    ),
  );

  return (
    <RecyclerListView
      layoutProvider={layoutProvider.current}
      dataProvider={dataProvider.current.cloneWithRows(newEvents)}
      rowRenderer={renderNews}
      onEndReached={() => console.log('load more cos end reached')}
      onEndReachedThreshold={200}
      forceNonDeterministicRendering
      canChangeSize
      style={{backgroundColor: 'rgba(0,0,0,0.9)', flex: 1}}
      //   onScroll={Animated.event(
      //     [
      //       {
      //         nativeEvent: {
      //           contentOffset: {
      //             y: scrollViewPos.current,
      //           },
      //         },
      //       },
      //     ],
      //     // {useNativeDriver: true},
      //   )}
      ref={flatListRef}
      //   renderFooter={wrap(renderFooter)}
      //   scrollViewProps={{
      //     refreshing,
      //     showsVerticalScrollIndicator: false,
      //     refreshControl: (
      //       <RefreshControl refreshing={refreshing} onRefresh={refresh} />
      //     ),
      //   }}
    >
      {/* <View style={{width: 200, hieght: 200, backgroundColor: 'red'}} /> */}
    </RecyclerListView>
  );
};

export default Component;
