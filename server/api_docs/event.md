# Event endpoints

These endpoints are for events on the app home page.



## POST `/event/new` (AUTHENTICATED)

Currently only admins can create events.

### Parameters:

```json 
{
    "pre_title": "{{PRE_TITLE}}",
    "title": "{{TITLE}}",
    "description": "{{DESCRIPTION}}",
    "lineup": [
        { "label": "{{LABEL}}", "description": "{{LINEUP_DESCRIPTION}}" }
        ...
    ],
    "location": "{{LOCATION}}",
    "is_virtual": {{VIRTUAL}},
    "virtual_link": "{{VIRTUAL_LINK}}?",
    "images": ["{{IMAGE_ID}}" ...],
    "event_start": {{EVENT_START}}
}
```

- `{{PRE_TITLE}}` is the part of text above the main title
- `{{TITLE}}` the main title
- `{{DESCRIPTION}}` the long form description of the event
- `{{VIRTUAL}}` should be set to true if the event is virtual
- `{{VIRTUAL_LINK}}` will only exist if `is_virtual` is true, otherwise it will be null / undefined.
- Lineup is optional
  - `{{LABEL}}` the time e.g. `2 pm` that the `{{LINEUP_DESCRIPTION}}` happens
- `{{LOCATION}}` the location of the event
- `{{EVENT_START}} ` is the time that the events start in seconds since the unix epoch



### OK response `200`:

```json
{
    "id": "{{EVENT_ID}}"
}
```

- `{{EVENT_ID}}` is id of the created announcement.

  

### Errors

```json
{
    "error": "The provided image hasn't been uploaded yet or isn't public",
    "error_code": "MISSING_OR_HIDDEN_IMAGE"
}
```

This will happen if one or more images don't exist or are private.

## GET `/event/find/{ANNOUNCEMENT_ID}`

### Parameters:

None, the only parameter is in the url.

### OK Response `200`:

```json
{
    "id": "{{EVENT_ID}}",
    "pre_title": "{{PRE_TITLE}}",
    "title": "{{TITLE}}",
    "description": "{{DESCRIPTION}}",
    "lineup": [
        { "lineup_time": "{{LINEUP_TIME}}", "description": "{{DESCRIPTION}}" }
        ...
    ],
    "location": "{{LOCATION}}",
    "is_virtual": {{VIRTUAL}},
    "virtual_link": "{{VIRTUAL_LINK}}?",
    "images": ["{{IMAGE_ID}}" ...],
    "event_start": {{EVENT_START}},
    "created_at": {{CREATED_AT}}
}
```

- `{{CREATED_AT}}`  and `{{EVENT_START}}` are both measured in the number of seconds since the UNIX epoch

### Errors

Will return a 404 if not found or if it's hidden.



## GET `/event/list`

Returns all the visible announcements in the order that the app should show them.

### Parameters:

None

### OK Response `200`:

```json
{
  "announcements": [{
    "id": "{{EVENT_ID}}",
    "pre_title": "{{PRE_TITLE}}",
    "title": "{{TITLE}}",
    "description": "{{DESCRIPTION}}",
    "lineup": [
        { "lineup_time": "{{LINEUP_TIME}}", "description": "{{DESCRIPTION}}" }
        ...
    ],
    "location": "{{LOCATION}}",
    "is_virtual": {{VIRTUAL}},
    "virtual_link": "{{VIRTUAL_LINK}}?",
    "images": ["{{IMAGE_ID}}" ...],
    "event_start": {{EVENT_START}},
    "created_at": {{CREATED_AT}}
}]
}
```

