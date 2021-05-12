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
import EventListCard from '../components/molecules/EventListCard';
import TestingMOLECULE from '../components/molecules/TestingDELETE2';

import {colors, sizes, fonts, icons, images} from '../constants';

const CommunityScreen = () => {
  return (
    <View style={{flex: 1}}>
      <Text>YOLO</Text>
      {/* <TestingMOLECULE
        data={[
          {url: 'https://source.unsplash.com/featured/university', id: 1234},
        ]}
        renderRow={(type, data) => <Text>yaaaa{data.url}</Text>}
      /> */}
      <TestingMOLECULE />
    </View>
  );
};

export default CommunityScreen;
