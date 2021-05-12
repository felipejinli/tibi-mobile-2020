import React from 'react';
import {View, Image, TouchableOpacity, Text} from 'react-native';
import PropTypes from 'prop-types';

import {fonts, sizes, colors, icons, images} from '../../../constants';
import FriendsAvatars from '../../atoms/FriendsAvatars';

const SquareDetailCard = ({item}) => {
  //   console.log('FJL SquareDetailCard DATA?!: ', item);
  const cardWidth = sizes.width * 0.44;
  return (
    <View
      style={{
        alignItems: 'center',
        justifyContent: 'center',
        marginHorizontal: sizes.xs,
      }}>
      <Image
        source={item.img}
        resizeMode="cover"
        style={{
          width: cardWidth,
          height: cardWidth,
          borderRadius: 5,
        }}
      />
      <View
        style={{
          width: '100%',
          alignSelf: 'flex-start',
        }}>
        <View
          style={{
            flexDirection: 'row',
            alignItems: 'center',
            justifyContent: 'space-between',
            marginTop: sizes.s,
          }}>
          <FriendsAvatars
            maxVisible={1}
            data={item.friendsInterested}
            // data={[
            //   {
            //     id: 0,
            //     img: images.profileAvatar,
            //   },
            //   {
            //     id: 1,
            //     img: images.profileAvatar,
            //   },
            //   {
            //     id: 2,
            //     img: images.profileAvatar,
            //   },
            //   {
            //     id: 3,
            //     img: images.profileAvatar,
            //   },
            //   {
            //     id: 4,
            //     img: images.profileAvatar,
            //   },
            // ]}
            // style={{marginTop: 20}}
          />
          <TouchableOpacity onPress={() => console.log('heartClick')}>
            <Image
              source={item.favorite ? icons.heartFill : icons.heartEmpty}
              resizeMode="contain"
              style={{width: 22, height: 22, marginRight: 8}}
            />
          </TouchableOpacity>
        </View>
        <Text
          style={{
            ...fonts.body3,
            marginTop: sizes.s,
          }}>
          {item.details}
        </Text>
      </View>
    </View>
  );
};

SquareDetailCard.propTypes = {
  data: PropTypes.shape({
    id: PropTypes.number.isRequired,
    details: PropTypes.string.isRequired,
    favorite: PropTypes.bool.isRequired,
    img: PropTypes.any.isRequired,
    friendsInterested: PropTypes.shape({
      friendId: PropTypes.number.isRequired,
      friendImg: PropTypes.any.isRequired,
    }),
  }),
};

// SquareDetailCard.defaultProps = {
//     item: {
//         id: 12035879,
//         details: 'ERROR: Please contact Tibi',
//         favorite: false,
//         img:
//     }
// }

export default SquareDetailCard;
