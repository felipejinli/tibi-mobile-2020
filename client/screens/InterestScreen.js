import React, {useState} from 'react';
import {TouchableOpacity, View, Text} from 'react-native';
import InterestCard from '../components/molecules/InterestCard';
import {fonts, colors, sizes} from '../constants';
import {useDispatch} from 'react-redux';
import {saveInterests} from 'state/action';

const InterestScreen = () => {
  const dispatch = useDispatch();

  const [categories, setCategories] = useState([
    {
      category: 'Career',
      image: 'https://source.unsplash.com/170x80/weekly?Career',
      selected: false,
    },
    {
      category: 'Health',
      image: 'https://source.unsplash.com/170x80/weekly?Health',
      selected: false,
    },
    {
      category: 'Food',
      image: 'https://source.unsplash.com/170x80/weekly?Food',
      selected: false,
    },
    {
      category: 'Music',
      image: 'https://source.unsplash.com/170x80/weekly?Music',
      selected: false,
    },
    {
      category: 'Photography',
      image: 'https://source.unsplash.com/170x80/weekly?Photography',
      selected: false,
    },
    {
      category: 'Dance',
      image: 'https://source.unsplash.com/170x80/weekly?Dance',
      selected: false,
    },
    {
      category: 'Fashion',
      image: 'https://source.unsplash.com/170x80/weekly?Fashion',
      selected: false,
    },
    {
      category: 'Technology',
      image: 'https://source.unsplash.com/170x80/weekly?Technology',
      selected: false,
    },
    {
      category: 'Culture',
      image: 'https://source.unsplash.com/170x80/weekly?Culture',
      selected: false,
    },
    {
      category: 'Night activities',
      image: 'https://source.unsplash.com/170x80/weekly?Night activities',
      selected: false,
    },
    {
      category: 'Arts',
      image: 'https://source.unsplash.com/170x80/weekly?Arts',
      selected: false,
    },
    {
      category: 'Writing',
      image: 'https://source.unsplash.com/170x80/weekly?Writing',
      selected: false,
    },
    {
      category: 'Games',
      image: 'https://source.unsplash.com/170x80/weekly?Games',
      selected: false,
    },
    {
      category: 'Outdoor',
      image: 'https://source.unsplash.com/170x80/weekly?Outdoor',
      selected: false,
    },
  ]);

  const handleChange = (index, selected) => {
    let newCategories = [...categories];
    newCategories[index].selected = selected;
    setCategories(newCategories);
  };

  return (
    <View
      style={{
        flex: 1,
        paddingVertical: '12.5%',
        paddingHorizontal: '3%',
        backgroundColor: 'black',
      }}>
      <View
        style={{
          flexDirection: 'row',
          justifyContent: 'space-between',
          marginBottom: '6.25%',
          marginHorizontal: '5%',
        }}>
        <Text style={[fonts.h1, {width: '70%'}]} numberOfLines={2}>
          Choose 3 or more interests
        </Text>
        <TouchableOpacity
          style={{flex: 1}}
          onPress={() => {
            /* Store interests in redux */
            const interests = [];
            categories.map((item, index) => {
              if (item.selected) {
                interests.push(item.category);
              }
            });
            console.log('Interests being saved', interests);
            dispatch(saveInterests(interests));
          }}>
          <Text
            style={{
              alignSelf: 'flex-end',
              color: colors.primary,
              fontSize: sizes.h2,
              fontWeight: 'bold',
            }}>
            Done
          </Text>
        </TouchableOpacity>
      </View>
      <View
        style={{
          flex: 1,
          flexDirection: 'column',
          flexWrap: 'wrap',
          justifyContent: 'space-evenly',
          alignItems: 'flex-start',
          alignContent: 'space-around',
        }}>
        {categories.map((item, index) => {
          return (
            <InterestCard
              key={index}
              index={index}
              label={item.category}
              image={item.image}
              onChange={handleChange}
            />
          );
        })}
      </View>
    </View>
  );
};

export default InterestScreen;
