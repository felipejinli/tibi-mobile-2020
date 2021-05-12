# Announcements endpoints

These endpoints are for announcements on the app home page.



## POST `/announcement/new` (AUTHENTICATED)

Currently only admins can create announcements.

### Parameters:

```json 
{
    "title": "{{TITLE}}",
    "subtitle": "{{SUBTITLE}}",
    "image": "{{IMAGE_ID}}"
}
```

- `{{IMAGE_ID}}` should be an id of an image from /image/new

### OK response `200`:

```json
{
    "id": "{{ANNOUNCEMENT_ID}}"
}
```

- `{{ANNOUNCEMENT_ID}}` is id of the created announcement.

  

### Errors

```json
{
    "error": "The provided image hasn't been uploaded yet or isn't public",
    "error_code": "MISSING_OR_HIDDEN_IMAGE"
}
```



## GET `/announcement/find/{ANNOUNCEMENT_ID}`

### Parameters:

None, the only parameter is in the url.

### OK Response `200`:

```json
{
    "id": "{{ANNOUNCEMENT_ID}}",
    "title": "{{TITLE}}",
    "subtitle": "{{SUBTITLE}}",
    "image": "{{IMAGE_ID}}",
    "created_at": {{CREATED_AT}}
}
```

- `{{CREATED_AT}}` number of seconds since the UNIX epoch

### Errors

Will return a 404 if not found or if it's hidden.



## GET `/announcement/list`

Returns all the visible announcements in the order that the app should show them.

### Parameters:

None

### OK Response `200`:

```json
{
  "announcements": [{
    "id": "{{ANNOUNCEMENT_ID}}",
    "title": "{{TITLE}}",
    "subtitle": "{{SUBTITLE}}",
    "image": "{{IMAGE_ID}}",
    "created_at": {{CREATED_AT}}
  }]
}
```

