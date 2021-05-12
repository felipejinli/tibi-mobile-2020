import common from './common';

export function sso() {
  return common.request('/auth/sso', 'POST');
}

export function sso_check(check_code) {
  return common.request('/auth/sso_check', 'POST', {check_code});
}

export function refresh(dispatch, auth_token) {
  return common.authenticated(dispatch, auth_token, '/auth/refresh', 'POST');
}

export default {sso, sso_check, refresh};
