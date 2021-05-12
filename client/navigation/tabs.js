import React from 'react';
import {View, Text, Image} from 'react-native';
import {createBottomTabNavigator} from '@react-navigation/bottom-tabs';

import {colors, icons} from '../constants';
import {Discover} from '../screens/';

const Tab = createBottomTabNavigator();

const tabOptions = {
  showLabel: false,
  style: {
    height: '10%',
    backgroundColor: colors.black,
  },
};

const Tabs = () => {
  return (
    <Tab.Navigator
      tabBarOptions={tabOptions}
      screenOptions={({route}) => ({
        tabBarIcon: ({focused}) => {
          const tintColor = focused ? colors.primary : colors.gray;

          switch (route.name) {
            case 'Discover':
              return (
                <Image
                  source={icons.discoverTab}
                  resizeMode="contain"
                  style={{
                    width: 32,
                    height: 32,
                    tintColor: tintColor,
                  }}
                />
              );
            case 'Search':
              return (
                <Image
                  source={icons.searchTab}
                  resizeMode="contain"
                  style={{
                    width: 32,
                    height: 32,
                    tintColor: tintColor,
                  }}
                />
              );
            case 'Messages':
              return (
                <Image
                  source={icons.messagesTab}
                  resizeMode="contain"
                  style={{
                    width: 32,
                    height: 32,
                    tintColor: tintColor,
                  }}
                />
              );
            case 'Community':
              return (
                <Image
                  source={icons.communityTab}
                  resizeMode="contain"
                  style={{
                    width: 32,
                    height: 32,
                    tintColor: tintColor,
                  }}
                />
              );
          }
        },
      })}>
      <Tab.Screen name="Discover" component={Discover} />
      <Tab.Screen name="Search" component={Discover} />
      <Tab.Screen name="Messages" component={Discover} />
      <Tab.Screen name="Community" component={Discover} />
    </Tab.Navigator>
  );
};

export default Tabs;
