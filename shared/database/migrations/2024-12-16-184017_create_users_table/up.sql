-- Your SQL goes here
CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    -- Unique ID
    name VARCHAR(64) NOT NULL,
    -- Unique name
    email VARCHAR(320) NOT NULL,
    -- Email
    password_hash VARCHAR(60) NOT NULL,
    -- Hashed password
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    -- Creation timestamp
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP -- Last update timestamp
);