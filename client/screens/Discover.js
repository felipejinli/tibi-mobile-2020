import React from 'react';
import {
  ScrollView,
  FlatList,
  TouchableOpacity,
  Text,
  StatusBar,
  View,
  StyleSheet,
  Image,
  SafeAreaView,
} from 'react-native';
import Icon from 'react-native-vector-icons/MaterialCommunityIcons';

import {colors, sizes, fonts, icons, images} from '../constants';
import HorizontalList from '../components/organisms/HorizontalList';
import WideCard from '../components/molecules/cards/WideCard';
import AnnouncementSwiper from '../components/organisms/AnnouncementSwiper';
import TestingDELETE from '../components/organisms/TestingDELETE';
import TestingMOLECULE from '../components/molecules/TestingDELETE';

const Discover = ({navigation}) => {
  // dummy data
  const [societies, setSocieties] = React.useState([
    {
      id: 0,
      img: images.society1,
      name: 'Artificial Intelligence',
    },
    {
      id: 1,
      img: images.society2,
      name: 'Beer Pong Society',
    },
    {
      name: 'Support Society',
      id: 49,
      img: {
        uri: 'http://dummyimage.com/242x230.png/ff4444/ffffff',
      },
    },
    {
      name: 'Legal Society',
      id: 47,
      img: {
        uri: 'http://dummyimage.com/239x214.png/5fa2dd/ffffff',
      },
    },
    {
      name: 'Human Resources Society',
      id: 33,
      img: {
        uri: 'http://dummyimage.com/169x144.png/ff4444/ffffff',
      },
    },
    {
      name: 'Support Society',
      id: 45,
      img: {
        uri: 'http://dummyimage.com/226x219.png/dddddd/000000',
      },
    },
    {
      name: 'Marketing Society',
      id: 29,
      img: {
        uri: 'http://dummyimage.com/102x236.png/ff4444/ffffff',
      },
    },
    {
      name: 'Training Society',
      id: 53,
      img: {
        uri: 'http://dummyimage.com/162x249.png/5fa2dd/ffffff',
      },
    },
    {
      name: 'Human Resources Society',
      id: 41,
      img: {
        uri: 'http://dummyimage.com/120x222.png/dddddd/000000',
      },
    },
    {
      name: 'Human Resources Society',
      id: 76,
      img: {
        uri: 'http://dummyimage.com/201x197.png/cc0000/ffffff',
      },
    },
    {
      name: 'Engineering Society',
      id: 30,
      img: {
        uri: 'http://dummyimage.com/233x230.png/ff4444/ffffff',
      },
    },
    {
      name: 'Support Society',
      id: 13,
      img: {
        uri: 'http://dummyimage.com/212x176.png/cc0000/ffffff',
      },
    },
  ]);
  const [events, setEvents] = React.useState([
    {
      id: 0,
      details: 'Wed 17 Jun • FREE',
      favorite: false,
      img: images.event1,
      friendsInterested: [
        {
          friendId: 123,
          friendImg: images.profileAvatar,
        },
        {
          friendId: 124,
          friendImg: images.profileAvatar,
        },
      ],
    },
    {
      id: 1,
      details: 'Wed 17 Jun • £10',
      favorite: true,
      img: images.event2,
      friendsInterested: [
        {
          friendId: 123,
          friendImg: images.profileAvatar,
        },
        {
          friendId: 124,
          friendImg: images.profileAvatar,
        },
      ],
    },
    {
      id: 2,
      details: 'Wed 17 Jun • FREE',
      favorite: false,
      img: images.event3,
      friendsInterested: [
        {
          friendId: 123,
          friendImg: images.profileAvatar,
        },
        {
          friendId: 124,
          friendImg: images.profileAvatar,
        },
        {
          friendId: 124,
          friendImg: images.profileAvatar,
        },
      ],
    },
    {
      id: 3,
      details: 'Wed 17 Jun • FREE',
      favorite: false,
      img: images.event1,
      friendsInterested: [
        {
          friendId: 123,
          friendImg: images.profileAvatar,
        },
        {
          friendId: 124,
          friendImg: images.profileAvatar,
        },
      ],
    },
    {
      id: 4,
      details: 'Wed 17 Jun • FREE',
      favorite: false,
      img: images.event2,
      friendsInterested: [
        {
          friendId: 123,
          friendImg: images.profileAvatar,
        },
        {
          friendId: 124,
          friendImg: images.profileAvatar,
        },
      ],
    },
  ]);
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
  const [announcements, setNewAnnouncements] = React.useState([
    {
      announcementImage: 'https://source.unsplash.com/collection/9045118/1',
      announcementTitle: '<h>Synergistic</h> actuating open architecture',
      announcementSubtitle: 'brand cross-platform architectures',
      _id: {
        $oid: '5f62fd8efc13ae33aa000000',
      },
    },
    {
      announcementImage: 'https://source.unsplash.com/collection/9045118/2',
      announcementTitle: '<h>Fully-configurable</h> dedicated capability',
      announcementSubtitle: 'incubate ubiquitous partnerships',
      _id: {
        $oid: '5f62fd8efc13ae33aa000001',
      },
    },
    {
      announcementImage: 'https://source.unsplash.com/collection/9045118/3',
      announcementTitle: 'Virtual background <h>process</h> improvement',
      announcementSubtitle: 'deploy impactful methodologies',
      _id: {
        $oid: '5f62fd8efc13ae33aa000002',
      },
    },
    {
      announcementImage: 'https://source.unsplash.com/collection/9045118/4',
      announcementTitle: 'Switchable analyzing hierarchy',
      announcementSubtitle: 'enhance customized systems',
      _id: {
        $oid: '5f62fd8efc13ae33aa000003',
      },
    },
    {
      announcementImage: 'https://source.unsplash.com/collection/9045118/5',
      announcementTitle: 'Balanced disintermediate orchestration',
      announcementSubtitle: 'embrace open-source methodologies',
      _id: {
        $oid: '5f62fd8efc13ae33aa000004',
      },
    },
  ]);

  return (
    <SafeAreaView style={styles.container}>
      <StatusBar barStyle="light-content" />
      <View style={{height: '8%'}}>
        <View
          style={{
            flexDirection: 'row',
            justifyContent: 'space-between',
            flex: 1,
            alignItems: 'center',
            marginHorizontal: sizes.l,
          }}>
          <TouchableOpacity onPress={() => navigation.openDrawer()}>
            <Image
              source={images.profileAvatar}
              resizeMode="contain"
              style={{height: 32, width: 32, borderRadius: 16}}
            />
          </TouchableOpacity>
          <Text style={{...fonts.headerTitle}}>ExploreUCL</Text>
          <Image
            source={icons.filter}
            resizeMode="contain"
            style={{width: 28, height: 28, tintColor: colors.white}}
          />
        </View>
      </View>
      {/* <TestingMOLECULE
        style={{width: sizes.width, height: sizes.height * 0.8}}
      /> */}
      <ScrollView showsVerticalScrollIndicator={false}>
        {/* <TestingDELETE /> */}
        {/* <TestingMOLECULE
          style={{width: sizes.width, height: sizes.height * 0.8}}
        /> */}
        <AnnouncementSwiper data={announcements} />
        <HorizontalList
          layout="square-detail"
          data={events}
          style={{
            marginTop: 40,
            // backgroundColor: 'red'
          }}
          title="Imagine all the people..."
        />
        <HorizontalList
          layout="circular"
          data={societies}
          style={{
            marginTop: 40,
            // backgroundColor: 'red',
          }}
          title="Trending Communities"
        />
        <HorizontalList
          layout="wide"
          data={newEvents}
          style={{marginTop: 40}}
          title="Weekend with Friends"
        />
        <View style={{width: '100%', height: 100}} />
      </ScrollView>
    </SafeAreaView>
  );
};

const styles = StyleSheet.create({
  container: {
    flex: 1,
    backgroundColor: colors.black,
  },
});

export default Discover;
