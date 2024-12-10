-- Your SQL goes here
CREATE TABLE authentification (
    id SERIAL PRIMARY KEY,
    name VARCHAR(32) UNIQUE NOT NULL,
    auth_url TEXT NOT NULL,
    token_url TEXT NOT NULL,
    client_id TEXT NOT NULL,
    client_secret TEXT NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);