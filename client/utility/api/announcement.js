import common from './common';

export function new_announcement(dispatch, auth_token, title, subtitle, image) {
  return common.authenticated(
    dispatch,
    auth_token,
    '/announcement/new',
    'POST',
    {title, subtitle, image},
  );
}

export function find(id) {
  return common.request('/announcement/find/' + encodeURIComponent(id), 'GET');
}

export function list() {
  return common.request('/announcement/list', 'GET');
}

export default {new_announcement, find, list};
