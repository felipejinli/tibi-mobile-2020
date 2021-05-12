import {createStore} from 'redux';
import {persistStore, persistReducer} from 'redux-persist';
import AsyncStorage from '@react-native-community/async-storage';
import autoMergeLevel2 from 'redux-persist/lib/stateReconciler/autoMergeLevel2';

function rootReducer(state = {}, action) {
  const newState = {...state};

  switch (action.type) {
    case 'AUTH_LOGIN':
      newState.auth_token = action.auth_token;
      newState.user = action.user;
      return newState;
    case 'AUTH_LOGOUT':
      newState.auth_token = null;
      newState.user = null;
      return newState;
    case 'SAVE_INTERESTS':
      newState.interests = action.interests;
      return newState;
    default:
      return state;
  }
}

const persistConfig = {
  key: 'root',
  storage: AsyncStorage,
  stateReconciler: autoMergeLevel2,
};

const pReducer = persistReducer(persistConfig, rootReducer);

export const store = createStore(pReducer);
export const persistor = persistStore(store);
