import React, {useEffect} from 'react';
import {createStackNavigator} from '@react-navigation/stack';
import {NavigationContainer, DefaultTheme} from '@react-navigation/native';
import {View, Button, Image} from 'react-native';
import {Provider, useSelector} from 'react-redux';
import {persistor, store} from 'state/store';
import {PersistGate} from 'redux-persist/lib/integration/react';

import {Discover, EventDetail, LoginScreen, InterestScreen} from './screens/';
import Tabs from './navigation/tabs';
import SideDrawer from './navigation/navigators/SideDrawer';
import {Avatar, Title, RightButton} from './components/atoms/Header';
import {images} from 'constants';
import SplashScreen from 'react-native-splash-screen';

const theme = {
  ...DefaultTheme,
  colors: {
    ...DefaultTheme.colors,
    border: 'transparent',
  },
};

// TODO: [TB-34] refactor these screens into their proper directory
const EventsListFullScreen = ({navigation}) => {
  return (
    <View style={{flex: 1, alignItems: 'center', justifyContent: 'center'}}>
      <Button
        onPress={() => navigation.goBack()}
        title="This is EventsListFull Screen. (MainTabs) => goBack in RootStack => (MainTabs)"
      />
    </View>
  );
};

const OnboardingScreen = ({navigation}) => {
  return (
    <View style={{flex: 1, alignItems: 'center', justifyContent: 'center'}}>
      <Button
        onPress={() => navigation.goBack()}
        title="This is Onboarding Screen. (Onboarding) => reset in RootStack => (SideDrawer)"
      />
    </View>
  );
};

const RootStack = createStackNavigator();

const App = () => {
  useEffect(() => {
    SplashScreen.hide();
  }, []);

  const authToken = useSelector((state) => state.auth_token);
  const interests = useSelector((state) => state.interests);

  console.log('Redux Persisted Interests', interests);
  if (!authToken) {
    return <LoginScreen />;
  } else if (!interests) {
    return <InterestScreen />;
  }

  return (
    <NavigationContainer>
      <RootStack.Navigator
        screenOptions={{
          headerShown: false,
          gestureEnabled: false,
        }}
        initialRouteName="SideDrawer">
        {/* <RootStack.Screen
          name="Discover"
          component={Tabs}
          //   options={{
          //     title: 'My home',
          //     headerStyle: {
          //       backgroundColor: '#f4511e',
          //     },
          //     headerTintColor: '#fff',
          //     headerTitleStyle: {
          //       fontWeight: 'bold',
          //     },
          //     headerShown: true,
          //   }}
        /> */}
        <RootStack.Screen name="SideDrawer" component={SideDrawer} />
        <RootStack.Screen
          name="EventsListFull"
          component={EventsListFullScreen}
          //   options={{
          //     headerLeft: () => <Avatar />,
          //     headerTitle: () => <Title style={{color: 'black'}} />,
          //     headerRight: () => <RightButton />,
          //     headerRightContainerStyle: {marginRight: 10},
          //     headerLeftContainerStyle: {marginLeft: 16},
          //     headerStyle: {
          //       backgroundColor: 'black',
          //       shadowColor: 'transparent',
          //     },
          //     headerShown: true,
          //   }}
        />
        <RootStack.Screen name="Onboarding" component={OnboardingScreen} />
      </RootStack.Navigator>
    </NavigationContainer>
  );
};

const LoadingScreen = () => {
  return (
    <View style={{backgroundColor: 'black', flex: 1, justifyContent: 'center'}}>
      <Image
        source={images.tibi}
        resizeMode="contain"
        style={{alignSelf: 'center'}}
      />
    </View>
  );
};

export default () => {
  return (
    <Provider store={store}>
      <PersistGate loading={<LoadingScreen />} persistor={persistor}>
        <App />
      </PersistGate>
    </Provider>
  );
};
