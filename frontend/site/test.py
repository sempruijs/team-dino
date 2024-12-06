import requests

# Define the API endpoint
url = "http://108.61.188.109:3030/users"

# Define the payload
data = {
    "user_id": "123e4567-e89b-12d3-a456-426614174000",
    "name": "John Doe",
    "email": "johndoe@example.com",
    "date_of_birth": "1990-01-01",
    "password": "very-special-password"
}

# Define the headers
headers = {
    "Content-Type": "application/json"
}

try:
    # Make the POST request
    response = requests.post(url, json=data, headers=headers)
    # Print the response
    print("Status Code:", response.status_code)
    print("Response Body:", response.text)
except requests.exceptions.ConnectionError as e:
    print("Error: Unable to connect to the server. Is it running?")
    print(e)
except Exception as e:
    print("An unexpected error occurred:")
    print(e)
