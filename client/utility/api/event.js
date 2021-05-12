import common from './common';

export function new_event(
  dispatch,
  auth_token,
  pre_title,
  title,
  description,
  lineup,
  location,
  is_virtual,
  virtual_link,
  images,
  event_start,
) {
  return common.authenticated(dispatch, auth_token, '/event/new', 'POST', {
    pre_title,
    title,
    description,
    lineup,
    location,
    is_virtual,
    virtual_link,
    images,
    event_start,
  });
}

export function find(id) {
  return common.request('/event/find/' + encodeURIComponent(id), 'GET');
}

export function list() {
  return common.request('/event/list', 'GET');
}

export default {new_event, find, list};
