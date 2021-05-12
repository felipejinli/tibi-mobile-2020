import isURL from 'validator/lib/isURL';

const createUriObject = (uri) => {
  return {uri: uri};
};

const handleImageProp = (img) => {
  if (isURL(img, {protocols: ['https', 'http'], require_protocol: true})) {
    console.log('valid img uri prop =>', createUriObject(img));
    return createUriObject(img);
  } else {
    throw new Error('Image prop has invalid URI');
  }
};

export {handleImageProp};
