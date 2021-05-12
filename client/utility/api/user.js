import common from './common';

export function info(dispatch, auth_token) {
  return common.authenticated(dispatch, auth_token, '/user/info', 'POST');
}

export default {info};
