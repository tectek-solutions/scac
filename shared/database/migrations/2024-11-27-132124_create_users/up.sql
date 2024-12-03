-- Your SQL goes here
CREATE TABLE Users (
    id SERIAL PRIMARY KEY,             -- Unique user ID
    name VARCHAR(50) UNIQUE NOT NULL, -- Unique username
    email VARCHAR(100) UNIQUE NOT NULL,   -- User email
    password_hash TEXT NOT NULL,       -- Hashed password
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP, -- Account creation date
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP -- Last update date
);