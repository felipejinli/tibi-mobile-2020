import React from 'react';
import {View, Text, Image, Button} from 'react-native';
import {createBottomTabNavigator} from '@react-navigation/bottom-tabs';
import {useSelector} from 'react-redux';

import {colors, icons} from '../../constants';
import {Discover, CommunityScreen, LoginScreen} from '../../screens/';

// TODO: [TB-36] Refactor Search Screen
const SearchScreen = ({navigation}) => {
  const name = useSelector((state) => state.user.given_name);
  return (
    <View style={{flex: 1, alignItems: 'center', justifyContent: 'center'}}>
      <Text style={{color: 'red'}}>{name}</Text>
      <Button
        onPress={() => navigation.navigate('EventsListFull')}
        title="This is Search Screen. (SearchScreen) => navigate('EventsListFull') from MainTabs => (EventsListFull)"
      />
    </View>
  );
};

const Tab = createBottomTabNavigator();

const tabOptions = {
  showLabel: false,
  style: {
    height: '10%',
    backgroundColor: colors.black,
  },
};

const MainTabs = () => {
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
      <Tab.Screen name="Search" component={SearchScreen} />
      <Tab.Screen name="Messages" component={LoginScreen} />
      <Tab.Screen name="Community" component={CommunityScreen} />
    </Tab.Navigator>
  );
};

export default MainTabs;
