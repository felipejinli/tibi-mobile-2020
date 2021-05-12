import React, {useState, useRef, useEffect} from 'react';
import {Dimensions, View, StyleSheet} from 'react-native';
import {RecyclerListView, DataProvider, LayoutProvider} from 'recyclerlistview';

function createNewDataProvider() {
  return new DataProvider((r1, r2) => {
    return r1 !== r2;
  });
}

export function DogList({data, renderRow}) {
  const [dataProvider, setDataProvider] = useState(
    createNewDataProvider().cloneWithRows(data),
  );
  const [dimensions] = useState(Dimensions.get('window'));
  const layoutProviderRef = useRef();

  layoutProviderRef.current = new LayoutProvider(
    (index) => 1,
    (type, dim) => {
      dim.width = dimensions.width / 3;
      dim.height = dimensions.width / 3;
    },
  );

  useEffect(() => {
    setDataProvider(createNewDataProvider().cloneWithRows(data));
  }, [data]);

  const computedStyles = styles(dimensions);
  return (
    <View style={computedStyles.container}>
      <RecyclerListView
        layoutProvider={layoutProviderRef.current}
        dataProvider={dataProvider}
        rowRenderer={renderRow}
        canChangeSize
        useWindowScroll
      />
    </View>
  );
}

const styles = ({height, width}) =>
  StyleSheet.create({
    container: {
      height,
      width,
    },
  });
