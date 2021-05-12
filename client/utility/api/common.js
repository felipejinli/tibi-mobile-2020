import action from 'state/action';

// TODO: make this come from an environment variable.
// This is solely used in request to prepend the provided endpoint url.
const API_BASE_URL = 'https://tibi.dewardt.uk';

// Represents an error (non 200 response) emitted from the server that was valid JSON.
// Contains an error code and/or a human readable error message.
// Also contains the HTTP status code (non 200).
export class ResponseError extends Error {
  constructor(status, code, message, ...params) {
    super('Response Error', ...params);

    this.errorType = 'ResponseError';
    this.status = status;
    this.code = code;
    this.message = message;
  }
}

// Represents an error emitted from the server that was NOT valid JSON.
// Contains the HTTP status code and the text response from the server.
// Do not show the user the text response as it has no guarantees about
// formatting. Although it should be logged in console.error.
//
// If this error is received an appropriate response is to say that
// there was a STATUS_CODE error from the server (where STATUS_CODE is
// the actual number of the code received).
//
// Hopefully this error should be uncommon as most errors should
// be properly formatted JSON.
export class ServerError extends Error {
  constructor(status, response, ...params) {
    super('Server Error', ...params);

    this.errorType = 'ServerError';
    this.status = status;
    this.response = response;
  }
}

// Represents an error that occured on the network connection e.g.
// broken connection.
export class NetworkError extends Error {
  constructor(...params) {
    super('Network Error');
  }
}

// Represent a request builder (without authentication)
// It will automatically convert network errors into the
// appropriate NetworkError class.
// If the status code is non-zero it will try to create a ResponseError
// is the message is valid JSON otherwise it will create a ServerError.
//
// If the method is POST then params are JSON encoded into the body.
// If the method is GET then the params are individually URI encoded
// and put into the format key=value&... then the resulting string is
// appended to the url with a `?` in between to signify they are GET params.
//
// extraHeaders is an optional field solely intended for use by authenticated
// to add the `Authorization` header.
export function request(url, method, params = {}, extraHeaders = {}) {
  let req;
  if (method === 'GET') {
    let queryparams = [];
    for (let key of Object.keys(params)) {
      queryparams.push(
        encodeURIComponent(key) + '=' + encodeURIComponent(params[key]),
      );
    }
    const querystring = queryparams.json('&');
    const uri = API_BASE_URL + url + '?' + querystring;
    req = fetch(uri, {
      method: 'GET',
      headers: {Accept: 'application/json', ...extraHeaders},
    });
  } else if (method === 'POST') {
    req = fetch(API_BASE_URL + url, {
      method: 'POST',
      headers: {
        Accept: 'application/json',
        'Content-Type': 'application/json;charset=UTF-8',
        ...extraHeaders,
      },
      body: JSON.stringify(params),
    });
  } else {
    throw new Error('Invalid HTTP method: ' + method);
  }

  return req.then((response, error) => {
    // The only error are ones due to the network.
    // There will be a repsponse even if the HTTP status code is non 200.
    if (error) {
      // TODO: Here it may be a good idea to send a message to the state marking the network
      // connection as poor/disconnected so that the user can see a bar explaining why
      // the app may be slow (it's their network's fault not ours).
      console.error(`Network error when contacting ${url}: ${error}`);
      throw NetworkError();
    }

    if (response.ok) {
      return response.json();
    } else {
      // Bad status code but we need to check to see if it's valid
      // json or not.
      // All branches here will reject (as the bad status code indicates an error).
      // but we need to decide with what error class they reject.
      return new Promise((_resolve, reject) => {
        response
          .json()
          .then((value) => {
            // Empty JSON means just status code was returned and nothing else.
            if (!value) {
              throw new Error();
            }
            // This will be caught and turned into a ServerError
            else {
              // At least one of these must be set otherwise the json is not a valid
              // error message (preferably in future both must be set).
              if (value.error || value.error_code) {
                // Reject the outer promise
                reject(
                  new ResponseError(
                    response.status,
                    value.error_code,
                    value.error,
                  ),
                );
              } else {
                // Effectively invalid JSON, will be converted to ServerError
                throw new Error();
              }
            }
          })
          // Here is where those errors are caught and converted.
          // This will also be called if the JSON was attempted to be parsed
          // but it was invalid.
          .catch(() => {
            // Need to access the text content of the response async as that
            // is the only available api.
            // It shouldn't fail as the JSON has already completed (hopefully).
            // In any case a failure here is a bit extreme and very unlikely in practise.
            response.text().then((text) => {
              reject(new ServerError(response.status, text));
            });
          });
      });
    }
  });
}

// Represents an authenticated request builder.
// This sets up a handler that checks for a 401 response from
// the server which indicates some issue with the auth_token
// if this happens it will dispatch a `LOGOUT` event forcing the
// user to log back in and retreive their auth token.
export function authenticated(dispatch, auth_token, url, method, params) {
  return request(url, method, params, {
    Authorization: 'Bearer ' + auth_token,
  }).catch((err) => {
    if (err instanceof ResponseError) {
      if (err.code.startsWith('AUTH_') && err.status === 401) {
        // In future we may wish to provide the reason but that will require some changes to state.
        console.error(`Authentication error [${err.code}]: ${err.message}`);
        dispatch(action.logout());
      }
    }

    // Rethrow the error as catch will get rid of the error by default.
    throw err;
  });
}

export default {
  request,
  authenticated,
  NetworkError,
  ResponseError,
  ServerError,
};
