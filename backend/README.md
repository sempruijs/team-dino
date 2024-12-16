# backend

Here are some examples how you can use the api with curl:

### Create user

```bash
curl -X POST http://localhost:3030/users \
    -H "Content-Type: application/json" \
    -d '{
        "user_id": "123e4567-e89b-12d3-a456-426614174000",
        "name": "John Doe",
        "email": "johndoe@example.com",
        "date_of_birth": "1990-01-01",
        "password": "very-special-password"
    }'
```

### authenticate user

Currently this returns a boolean but later it will return a jwt.

```bash
curl -X POST http://127.0.0.1:3030/authenticate \
    -H "Content-Type: application/json" \
    -d '{"email": "johndoe@example.com", "password": "very-special-password"}'
```

### create ticket

This will only create a ticket when the house is availible within the selected dates.
There will never be 2 users that rent the same place on the same date.

```bash
curl -X POST http://localhost:3030/create_ticket \
     -H "Content-Type: application/json" \
     -d '{
           "ticket_id": "12998890-9d83-4dd1-b9fe-c6631c1042ee",
           "user_id": "123e4567-e89b-12d3-a456-426614174000",
           "start_date": "2024-07-01",
           "end_date": "2024-07-07",
           "place_id": "10000090-9d83-4dd1-b9fe-c6631c1042ee",
         }'
```

### create place

This is for creating new places on the camping.

```bash
curl -X POST http://localhost:3030/create_place \
     -H "Content-Type: application/json" \
     -d '{
           "place_id": "21118890-9d83-4dd1-b9fe-c6631c1042ee",
           "house_number": 42
         }'
```

### Delete place

You can delete a place. Replace the place_id with your own place_id.

```bash
curl -X DELETE http://localhost:3030/places/{place_id}

curl -X DELETE http://localhost:3030/places/{place_id} \
  -H "Authorization: Bearer {your_jwt_token}"
```

### recieve all places

This will recieve all places that currently exist on the camping regardless of their availebility.

```bash
curl -X GET http://localhost:3030/places
```

### get places with date filter

this will recieve availible places based on a start and end date.

```bash
curl "http://localhost:3030/places/available?start_date=2024-12-20&end_date=2024-12-25"
```

### Check if license plate is valid
a license plate is valid when it exists and the assosiated user has a valid ticket. A ticket is valid when the current date is within the range of the start and end date of the ticket.

```bash
curl -X GET "http://localhost:3030/check_license_plate/ABC123"
```

### Check if card exists

```bash
curl -X GET "http://localhost:3030/check_card/bla-bla-bla"
```

### Get user by uuid

```bash
curl -X GET http://localhost:3030/users/<UUID>
```

So with an example user:

```bash
curl -X GET http://localhost:3030/get_user/123e4567-e89b-12d3-a456-426614174000
```


### Add a license plate to a user

```bash
curl -X POST http://localhost:3030/license_plates \
     -H "Content-Type: application/json" \
     -d '{
           "user_id": "123e4567-e89b-12d3-a456-426614174000",
           "license_plate": "ABC123"
         }'
```

### Add card to a user

```bash
curl -X POST http://localhost:3030/cards \
     -H "Content-Type: application/json" \
     -d '{
           "user_id": "123e4567-e89b-12d3-a456-426614174000",
           "card_id": "bla-bla-bla"
         }'
```