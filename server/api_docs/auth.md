# Authentication endpoints

These endpoints related to authenticating with tibi and they also abstract the oauth situation with the UCL api.

## A note on authenticated endpoints

Most api endpoints will require authentication in the form of an `auth_token`. These endpoints are marked as `AUTHENTICATED {{METHOD}} {{PATH}}`.

In order to use these points you **must** set the `Authorization` header to `Bearer {{AUTH_TOKEN}}`. In full the header should be `Authorization: Bearer {{AUTH_TOKEN}}` where `{{AUTH_TOKEN}}` are the ones provided from logging in at `/auth/sso`.

Any endpoint marked as `AUTHENTICATED` may result in one of the following errors (including any additional errors specified by the documentation for that particular end point). These errors will not be documented on any particular endpoint, only here.

The HTTP status code will be `401` for these errors. The field `error` is supposed to be a human readable message and can change at any time. The field `error_code` is meant to be machine readable and is guaranteed to be stable (not change).



```json
{
    "error": "You are not logged in",
    "error_code": "AUTH_TOKEN_MISSING"
}
```

Happens when the auth token is missing. This can occur if the app has not been programmed to provide an authentication token such as if it was unknown that the specified endpoint was authenticated or if the app attempted to perform actions to an API endpoint despite the user not being logged in.



```json
{
    "error": "Your login session has expired",
    "error_code": "AUTH_TOKEN_EXPIRED"
}
```

Happens when the auth token has been used for too long without refreshing it. This could happen if the app is not properly refreshing the auth token or if the user has not logged into their app for a long period of time.



```json
{
    "error": "Your authentication token is invalid",
    "error_code": "AUTH_TOKEN_INVALID"
}
```

Happens when the authentication token is either improperly formatted or when cryptographically verifying the token it could not be verified. The app should logout the user and re-attempt a log in.



## POST `/auth/sso`

### Parameters:

 `None`

### OK response `200`:

```json
{
    "redirect_url": "{{URL}}",
    "check_code": "{{CHECK_CODE}}",
    "timeout": {{TIMEOUT}}
}
```

- `{{URL}}` is the url of the location that the app should redirect the user in order to to sign in to their UCL account in the user's browser.

- `{{CHECK_CODE}}` is the code that the front-end should use in order to regularly poll the server to detect whether the user has been authenticated with UCL's single sign on as when the process is complete the server will be notified but there is no easy way in the HTTP protocol for the server to notify the client (in HTTP clients must initiate requests). This means that there has to be the relatively inefficient polling method. In future a possible optimisation will be to use a long polling method which is very simple on the client side and in general uses very few resources as there is only on request. I currently have no experience implementing this in actix so this will be a future thing to look into.

  The check_code is actually just the `state` token provided to the UCL api used to uniquely track a particular sign on request. This is irrelevant to the client api and should be considered an implementation detail.

  Since this code may be used to retrieve the `auth_token`, if a malicious party was able to gain access to this token during the brief window when the user is logging on, they may be able to retrieve the `auth_token` so it should be treated just as securely as the `auth_token` is. In future it may be wise to have this code be different to the `state` token for security purposes but currently as far as everything seems to be implemented it should be very difficult for this to be intercepted (UCL api is contacted over HTTPS).

- `{{TIMEOUT}}` (integer) the unix timestamp in milliseconds when the server will consider the request to have timed out. Even if the authentication succeeds after that point the server will have already deleted the records and will be unable to accept the authentication from the UCL api. After this point the client should only poll `/auth/sso_check` one more time and if it fails with `404` show an error to the user saying that the sign on timed out.

### Errors

Currently none



## POST `/auth/sso_check`

### Parameters

```json
{
    "check_code": "{{CHECK_CODE}}"
}
```

### OK response `200`:

Several different responses may be returned depending on the status field. In the case of `AUTHENTICATED` , `ERROR` , `PROCESSING` the server will extend the timeout period to allow some tolerance so that the client is unlikely to miss a status change from `WAITING`. This extension is currently unspecified but should be about a few minutes, so as long as the polling time of the client app is significantly less than that, even in the worst case where the message comes in just before the timeout, you will still get the update.

```json
{
    "status": "AUTHENTICATED",
    "auth_token": "{{AUTH_TOKEN}}"
}
```

This response is when the user successfully completed a single sign on **and** the server has successfully contacted the UCL api and the user's data is inserted into the database (if it didn't exist already). This is only possible if the user actually authenticated with the server and didn't just fake the callback url manually.

- `AUTH_TOKEN` the authentication token that can be used to authenticate the client to other API endpoints.

Note: in future it may be a good idea for this method to return a user object instead of requiring the delay and round-trip to `/user/info`.

Once `AUTHENTICATED` has been sent to the client any subsequent call will be as if the request timed out as the endpoint will return a `404` error.

OR

```json
{
    "status": "WAITING"
}
```

The server has received no update from the UCL SSO but it has not timed out yet.

OR

```json
{
    "status": "PROCESSING"
}
```

The server has received a successful authentication message to the provided callback URL but it has not yet contacted the UCL api to fetch the user data required to insert the user into the tibi database. At this point the app should show the user a message indicating success but that the server is processing the request. If the user faked the callback url the server will not know at this point this will be checked before `AUTHENTICATED` is returned as the status.

OR

```json
{
    "status": "ERROR",
    "error": "{{ERROR}}"
}
```

The server received a message from the UCL SSO but it was an error such as the user declined to authenticate the app. Another error occurs if the user is not a student such as if they are a teacher.

- `{{ERROR}}` is a human readable error explaining what went wrong with the single sign on. The app should display this error to the user.



### Errors

- `404` the server has no record of the specified `check_code`. This could occur if the check code has expired or if it never existed which is unlikely if you got the check code from `/auth/sso`. If the client app receives this code it should check to see if the timeout has passed, if it hasn't passed then this indicates some form of issue and it should be logged. If the timeout has in fact passed then this is the expected response.



## POST  `/auth/refresh` (AUTHENTICATED)

The client side should semi-regularly refresh the auth token to prevent it expiring.  Once a day is a reasonable refresh rate. On startup the app should immediately check the last time it refreshed the token and if it is longer than a day it should call this endpoint almost as soon as it loads / connects to the internet.

### Parameters

None

### OK response `200`:

```json
{
    "auth_token": "{{AUTH_TOKEN}}"
}
```

If the client received a successful response then this returned auth token should be used instead of the old one.



The only errors returned are those related to lack of proper authentication. See `A note on authenticated endpoints`.



## GET `/auth/sso_callback` (PRIVATE)

This endpoint **should not be used by the client app**. It is used by the UCL api to return the user data when the SSO authenticated happened (successful or not). The user will be re-directed to this page by the UCL api rather than the api directly contacting it.

### Parameters

- `state` An opaque, unique and cryptographically secure random string that is generated by the tibi api server to track an SSO request.
- `result` A string indicates the success (or not) of the SSO.
- `code` The OAuth token that allows tibi to access the UCL api on the user's behalf.
- `client_id` The api token id for our app (tibi).

### OK response `200`:

A rendered HTML page saying the user can go back to `tibi` as they have been authenticated (not really, at this point the server is still processing their authentication but they don't need to know that as the app will tell them and in most cases it will be very quick and by the time they re-enter the app it should be done).
