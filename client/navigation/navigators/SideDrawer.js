import React from 'react';
import {createDrawerNavigator} from '@react-navigation/drawer';
import {Button, View} from 'react-native';

import MainTabs from './MainTabs';

// TODO: [TB-35] Refactor Settings Screen
const SettingsScreen = ({navigation}) => {
  return (
    <View style={{flex: 1, alignItems: 'center', justifyContent: 'center'}}>
      <Button
        onPress={() => navigation.goBack()}
        title="This is Settings Screen. (SettingsScreen) => goBack in SideDrawer => (original Tab were SideDrawer opened)"
      />
    </View>
  );
};

const Drawer = createDrawerNavigator();

const SideDrawer = () => {
  return (
    <Drawer.Navigator initialRouteName="MainTabs">
      <Drawer.Screen name="MainTabs" component={MainTabs} />
      <Drawer.Screen name="Settings" component={SettingsScreen} />
    </Drawer.Navigator>
  );
};

export default SideDrawer;
