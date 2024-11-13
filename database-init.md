querries for the sql database:

-- users table
CREATE TABLE users (
    user_id UUID PRIMARY KEY,
    name TEXT NOT NULL,
    date_of_birth DATE NOT NULL,
    email TEXT UNIQUE NOT NULL,
    license_plate JSONB
);

-- tickets table
CREATE TABLE tickets (
    ticket_id UUID PRIMARY KEY,
    user_id UUID REFERENCES users(user_id),
    start_date DATE NOT NULL,
    end_date DATE NOT NULL,
    house_number INTEGER NOT NULL CHECK (house_number >= 0)
);
