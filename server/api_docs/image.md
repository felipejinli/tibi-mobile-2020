# Image endpoints

These endpoints are for uploading and retrieving permissions.

Currently to simplify the process only admins are allowed to upload photos as this is the minimum requirement for creating announcements and events. In future this api will be revisited and great care will be taken to ensure that users can't just use our api as a way of storing any image they want.



## POST `/image/new` (AUTHENTICATED)

Currently the only users who will be allowed to upload are admins.

The uploaded images default to private and are only accessible by users who uploaded them.

If the user wished to make them public they can call `/image/permissions` to update the permissions.

(TODO: in future figure out if it's easy to provide some metadata at upload time, e.g. public/private).

The end point accepts a multi part upload.

### Parameters:

The image files as a multi part upload in the body of the request (form data).

`Content-Type: multipart/form-data`.

The mime type for the image must be correctly set.

Any form data field which isn't an image if is ignored.

Multiple images are allowed and the system will keep track of the provided form data key.

### OK response `200`

```json
{
    "images": {
        "{{FORM_DATA_KEY}}": {
            "original_name": "{{ORIGINAL_NAME}}",
            "id": "{{IMAGE_ID}}"
        }
    }
}
```

- `{{FORM_DATA_KEY}}` is the key of the image in when the form data was sent to the server
- `{{ORIGINAL_NAME}}` is the original file name of the provided image
- `{{IMAGE_ID}}` is the unique ID of the image (can be used to retrieve the image contents)

### Errors

```json
{
    "error": "You do not have sufficient privileges to upload this image",
    "error_code": "LACKS_PERMISSIONS"
}
```

If the user does not have the correct permissions a 403 status code will be returned with the provided error.

## POST `/image/permissions` (AUTHENTICATED) - UNIMPLEMENTED TODO

Update the permissions of an image.

The authenticated user must have `WRITE` permissions on the image (or be an admin).

### Parameters (WORK IN PROGRESS):

```json
{
    "id": "{{IMAGE_ID}}",
    "public?": "boolean",
    "write?": {
        "add?": [{{USER_ID}}]
    },
    "read?": {
        "add"
    }
    
}
```



## GET `/image/public/{IMAGE_ID}`

Returns the image with that id if it exists and is public otherwise 404.