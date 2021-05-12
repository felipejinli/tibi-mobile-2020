export function logout() {
  return {type: 'AUTH_LOGOUT'};
}

export function login(auth_token, user) {
  return {type: 'AUTH_LOGIN', auth_token, user};
}

export function saveInterests(interests) {
  return {type: 'SAVE_INTERESTS', interests};
}
