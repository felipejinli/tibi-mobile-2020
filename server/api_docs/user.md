# User endpoints

These endpoints relate to accessing or modifying user information.

## Common definitions

Throughout the user api spec there are several common definitions:

- `USER_ID`: The database id of the user, commonly used when identifying users for operations. This field comes from the UCL api but is called `upi`.

- `GIVEN_NAME`: The given name (this field is originally supplied by the UCL api).

- `FULL_NAME`: The full name (this field is originally supplied by the UCL api).

- `USERNAME`: The username (this field is originally supplied by the UCL api).

- `EMAIL`: The user's email (this field is originally supplied by the UCL api).

- `DEPARTMENT`: The user's department. This may be null as not all users have a specified department (this field is originally supplied by the UCL api).

  


## POST `/user/info` (AUTHENTICATED)

Retrieves information for the current user (based on the auth_token supplied).

### Parameters

None

### OK response `200`:

```json
{
    "user": {
        "id": "{{USER_ID}}",
        "given_name": "{{GIVEN_NAME}}",
        "full_name": "{{FULL_NAME}}",
        "username": "{{USERNAME}}",
        "email": "{{EMAIL}}",
        "department": "{{DEPARTMENT}}"
    }
}
```

