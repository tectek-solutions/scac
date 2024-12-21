-- Your SQL goes here
CREATE TABLE users (
    id SERIAL PRIMARY KEY, -- Unique ID
    name TEXT UNIQUE NOT NULL, -- Unique name
    email TEXT UNIQUE NOT NULL, -- Email
    password_hash TEXT NOT NULL, -- Hashed password
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP, -- Creation timestamp
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP -- Last update timestamp
);
