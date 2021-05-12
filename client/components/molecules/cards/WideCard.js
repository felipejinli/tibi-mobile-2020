import React from 'react';
import {View, Image, TouchableOpacity, Text} from 'react-native';
import PropTypes from 'prop-types';

import {fonts, sizes, colors, icons, images} from '../../../constants';

const WideCard = ({item}) => {
  //   console.log('FJL WideCard DATA?!: ', item);
  const cardWidth = sizes.width * 0.6;
  const cardHeight = sizes.width * 0.45;
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
        style={{width: cardWidth, height: cardHeight, borderRadius: 5}}
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
          <View>
            <Text
              style={{
                ...fonts.h5,
                maxWidth: cardWidth * 0.75,
              }}
              numberOfLines={1}>
              {item.name}
            </Text>
            <Text
              style={{
                ...fonts.body3,
                marginTop: sizes.xs,
              }}
              numberOfLines={1}>
              {item.details.date} • {item.details.location} •{' '}
              {item.details.price}
            </Text>
          </View>
          <TouchableOpacity onPress={() => console.log('heartClick')}>
            <Image
              source={item.favorite ? icons.heartFill : icons.heartEmpty}
              resizeMode="contain"
              style={{width: 22, height: 22, marginRight: 8}}
            />
          </TouchableOpacity>
        </View>
      </View>
    </View>
  );
};

// TODO: proptypes based on Discover.js' newEvents state
// WideCard.propTypes = {
//   data: PropTypes.shape({
//     id: PropTypes.number.isRequired,
//     details: PropTypes.string.isRequired,
//     favorite: PropTypes.bool.isRequired,
//     img: PropTypes.any.isRequired,
//     name: PropTypes.string.isRequired,
//   }),
// };

// WideCard.defaultProps = {
//     item: {
//         id: 12035879,
//         details: 'ERROR: Please contact Tibi',
//         favorite: false,
//         img:
//     }
// }

export default WideCard;
