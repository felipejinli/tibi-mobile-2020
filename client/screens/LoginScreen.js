import React, {useState} from 'react';
import {TouchableOpacity, Text, View, Alert, Linking} from 'react-native';
import api from 'utility/api';
import InAppBrowser from 'react-native-inappbrowser-reborn';
import {useDispatch} from 'react-redux';
import {login} from 'state/action';

// ! prevent double tap calling API multiple times

const LoginScreen = () => {
  const [message, setMessage] = useState(null);
  const dispatch = useDispatch();

  async function openLink(redirectUrl) {
    try {
      //   const url = 'https://www.bbc.com';
      const url = redirectUrl;
      if (await InAppBrowser.isAvailable()) {
        const result = await InAppBrowser.open(url, {
          // iOS Properties
          dismissButtonStyle: 'cancel',
          preferredBarTintColor: '#453AA4',
          preferredControlTintColor: 'white',
          readerMode: false,
          animated: true,
          modalPresentationStyle: 'fullScreen',
          modalEnabled: true,
          enableBarCollapsing: false,
          // Android Properties
          showTitle: true,
          toolbarColor: '#6200EE',
          secondaryToolbarColor: 'black',
          enableUrlBarHiding: true,
          enableDefaultShare: true,
          forceCloseOnRedirection: false,
        });
      } else {
        Linking.openURL(url);
      }
    } catch (error) {
      Alert.alert(error.message);
    }
  }

  const signIn = () => {
    console.log('FJL entered signIn');
    api.auth.sso().then((res, err) => {
      console.log('FJL entered inside signIn');
      if (err) {
        console.error(err);
        return;
      }
      openLink(res.redirect_url);
      const sso_check = () => {
        api.auth.sso_check(res.check_code).then((check_res, check_err) => {
          if (err) {
            setMessage('Sign-on timed out. Please try again.');
            console.error(err);
            return;
          }
          switch (check_res.status) {
            case 'AUTHENTICATED':
              //   TODO: enable const for sign on button
              InAppBrowser.close();
              console.log(check_res);
              api.user.info(dispatch, check_res.auth_token).then((res, err) => {
                if (err) {
                  console.error(err);
                  setMessage(
                    "There's an error getting your information. Please try again later.",
                  );
                  return;
                }
                dispatch(login(check_res.auth_token, res.user));
              });
              return;
            case 'WAITING':
              setMessage('UCL Sign-On contact failed. Try again.');
              break;
            case 'PROCESSING':
              setMessage('We are processing your sign-on request');
              break;
            case 'ERROR':
              setMessage(check_res.error);
              return;
            default:
              console.error('Got invalid check_res status', check_res);
          }
          setTimeout(sso_check, 5000);
        });
      };
      sso_check();
      console.log('/auth/sso:', res, err);
    });
  };

  return (
    <View style={{flex: 1, justifyContent: 'center', alignItems: 'center'}}>
      <TouchableOpacity
        onPress={signIn}
        // onPress={() => console.log('pressed')}
        style={{height: 200, width: 200, backgroundColor: 'red'}}>
        <Text>Sign In</Text>
      </TouchableOpacity>
    </View>
  );
};

export default LoginScreen;
