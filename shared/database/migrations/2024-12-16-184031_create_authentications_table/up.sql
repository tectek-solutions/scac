-- Your SQL goes here
CREATE TABLE authentications (
    id SERIAL PRIMARY KEY,
    name VARCHAR(32) UNIQUE NOT NULL,
    authorization_url TEXT NOT NULL,
    authorization_http_parameters JSONB NOT NULL,
    token_url TEXT NOT NULL,
    token_url_http_parameters JSONB NOT NULL,
    client_id TEXT NOT NULL,
    client_secret TEXT NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);