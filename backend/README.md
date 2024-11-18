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
           "license_plate": ["ABC123", "XYZ789"],
           "tickets": []
         }'
```

### create ticket

```bash
curl -X POST http://localhost:3030/create_ticket \
     -H "Content-Type: application/json" \
     -d '{
           "ticket_id": "12998890-9d83-4dd1-b9fe-c6631c1042ee",
           "user_id": "123e4567-e89b-12d3-a456-426614174000",
           "start_date": "2024-07-01",
           "end_date": "2024-07-07",
           "house_number": 42
         }'
```

### Check if license plate exists

```bash
curl -X GET "http://localhost:3030/check_license_plate/ABC123"
```


### Get user by uuid

```bash
curl -X GET http://localhost:3030/get_user/<UUID>
```

So with an example user:

```bash
curl -X GET http://localhost:3030/get_user/123e4567-e89b-12d3-a456-426614174000
```
