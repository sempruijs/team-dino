-- users table
CREATE TABLE users (
    user_id UUID PRIMARY KEY,
    name TEXT NOT NULL,
    date_of_birth DATE NOT NULL,
    email TEXT UNIQUE NOT NULL,
    password TEXT NOT NULL
);

-- tickets table
CREATE TABLE tickets (
    ticket_id UUID PRIMARY KEY NOT NULL,
    user_id UUID NOT NULL REFERENCES users(user_id) ON DELETE CASCADE,
    start_date DATE NOT NULL,
    end_date DATE NOT NULL,
    place_id UUID NOT NULL,
);

-- places table
CREATE TABLE places (
    place_id UUID PRIMARY KEY NOT NULL,
    house_number INTEGER NOT NULL
);

-- license plate table
CREATE TABLE license_plates (
    plate_id SERIAL PRIMARY KEY,
    user_id UUID NOT NULL REFERENCES users(user_id) ON DELETE CASCADE,
    license_plate TEXT NOT NULL UNIQUE
);

-- create cards table
CREATE TABLE cards (
    id SERIAL PRIMARY KEY,
    user_id UUID NOT NULL REFERENCES users(user_id) ON DELETE CASCADE,
    card_id TEXT NOT NULL UNIQUE
);