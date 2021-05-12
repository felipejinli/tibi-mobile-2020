import React from 'react';
import {Text, Image, View} from 'react-native';

import {colors, fonts, sizes, images} from '../../constants';
import PropTypes from 'prop-types';

const FriendsAvatars = ({maxVisible, style, data}) => {
  function renderFriendsAvatars(maxVisible, data) {
    if (data.length <= maxVisible) {
      //   console.log('FJL renderFriendsAvatars');
      return data.map((item, index) => (
        <React.Fragment key={item.friendId}>
          <Image
            source={item.friendImg}
            resizeMode="cover"
            style={[
              {
                height: 36,
                width: 36,
                borderRadius: 18,
                borderWidth: 3,
                borderColor: 'black',
              },
              index === 0 ? {} : {marginLeft: -10},
            ]}
          />
        </React.Fragment>
      ));
    } else if (data.length > maxVisible) {
      //   console.log('renderFriendsAvatar branch else if index<maxVisible');
      return (
        <>
          {data.map((item, index) => {
            // console.log(item.friendId);
            if (index < maxVisible) {
              return (
                <React.Fragment key={item.friendId}>
                  <Image
                    source={item.friendImg}
                    resizeMode="cover"
                    style={[
                      {
                        height: 36,
                        width: 36,
                        borderRadius: 18,
                        borderWidth: 3,
                        borderColor: 'black',
                        zIndex: -index,
                      },
                      index === 0 ? {} : {marginLeft: -10},
                    ]}
                  />
                </React.Fragment>
              );
            } else if (index === maxVisible) {
              return (
                <React.Fragment key={item.friendId}>
                  <View
                    style={{
                      marginLeft: -10,
                      height: 36,
                      width: 36,
                      borderRadius: 18,
                      borderWidth: 3,
                      borderColor: 'black',
                      backgroundColor: 'white',
                      justifyContent: 'center',
                      alignItems: 'flex-end',
                      paddingRight: 6,
                      zIndex: -index,
                    }}>
                    <Text
                      style={{
                        ...fonts.body4,
                        paddingTop: sizes.xs,
                      }}>
                      +{data.length - maxVisible}
                    </Text>
                  </View>
                </React.Fragment>
              );
            }
          })}
        </>
      );
    }
  }
  return (
    <View style={[{flexDirection: 'row', alignItems: 'center'}, style]}>
      {/* {console.log('FJL what is friends avatar DATAAA:', data)} */}
      {data.length ? renderFriendsAvatars(maxVisible, data) : null}
    </View>
  );
};

FriendsAvatars.propTypes = {
  maxVisible: PropTypes.number.isRequired,
  style: PropTypes.object,
  data: PropTypes.arrayOf(
    PropTypes.shape({
      friendId: PropTypes.number.isRequired,
      friendImg: PropTypes.any.isRequired,
    }),
  ),
};

export default FriendsAvatars;
