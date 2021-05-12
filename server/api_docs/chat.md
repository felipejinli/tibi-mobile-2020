# Chat endpoints

These endpoints are for chat both live and accessing historical information



## WS `/chat/connect`

This endpoint is a websocket connection. The secure version of websocket should be used (`wss://`).

After the connection is open you must send the following message:

```json
{
    "auth_token": "{{AUTH_TOKEN}}"
}
```

If there is an issue you will receive a JSON encoded message with `error` and `error_code` fields just like what authenticated endpoints return so looks at the `auth` docs for more detail.

Once the authentication token is received by the server it will response with either the error detailed above or a message as below:

```json
{
    "status": "CONNECTED"
}
```

Once this message is received only then should the app consider the connection made. Any live chat messages sent before this will not be sent to the client, only after `CONNECTED` is sent can you be sure that live messages will arrive.

Incoming messages will have the format:

```json
{
    "message": {
        "id": {{MESSAGE_ID}},
        "message": "{{MESSAGE_CONTENT}}",
        "poster": "{{POSTER_ID}}",
        "room": "{{ROOM_ID}}",
        "timestamp": {{UNIX_TIMESTAMP}}
    }
}
```

To send a message you must send the following request:

```json
{
    "room": "{{ROOM_ID}}",
    "messages": "{{MESSAGE_CONTENT}}"
}
```

If there is an error e.g. the user is not in the provided room then nothing will be returned although an error will be logged in the server logs. This is a small security feature.

If there is an internal server error then the connection will be closed. At which point it's safe to assume that the server will not be able to process messages anyway so a closed connection is probably the best UI in this case.



## POST `/chat/create_dm` (AUTHENTICATED)

### Parameters

```json
{
    "other": "{{USER_ID}}"
}
```

- `{{USER_ID}}` the id of the user with whom you wish to create a direct message room.

### OK Response `200`

No body

### Errors

- `NOT_FOUND` (404) the other user didn't exist



### POST `/chat/room_info` (AUTHENTICATED)

### Parameters

```json
{
    "room_id": "{{ROOM_ID}}"
}
```

- `{{ROOM_ID}}` the id of the room about which you want info.

### Response

```json
{
    "room_name": "{{ROOM_NAME}}",
    "occupants": ["{{USER_ID}}", "{{USER_ID}}"]
}
```

- `{{ROOM_NAME}}` the name of the room as a string, or null if the room has no name.
- occupants is a list of user ids (strings) of the users who are in the room

### Errors

- `NOT_FOUND` (404) the room doesn't exist or the current user is not an occupant of the specified room



### POST `/chat/retrieve` (AUTHENTICATED)

### Parameters

```json
{
    "after": {{MESSAGE_ID}}
}
```

- `{{MESSAGE_ID}}` the message id that has a lower id than all the messages you want set this to 0 if you want all messages. Note the endpoint will not include the specified message id in the returned messages.

### OK response (`200`)

```json
{
    "room_name": "{{ROOM_NAME}}",
    "occupants": [{
        "id": {{MESSAGE_ID}},
        "message": "{{MESSAGE_CONTENT}}",
        "poster": "{{POSTER_ID}}",
        "room": "{{ROOM_ID}}",
        "timestamp": {{UNIX_TIMESTAMP}}
    }, ...]
}
```

All the messages returned are from rooms that the authenticated user is in and all the messages have a greater ID than the `after` parameter (greater ID means that the message is newer).

