import React from 'react';
import {TouchableOpacity, Image} from 'react-native';
import {images} from '../../../constants';

function Avatar() {
  return (
    <TouchableOpacity>
      <Image
        source={images.profileAvatar}
        resizeMode="contain"
        style={{height: 32, width: 32, borderRadius: 16}}
      />
    </TouchableOpacity>
  );
}

export default Avatar;
