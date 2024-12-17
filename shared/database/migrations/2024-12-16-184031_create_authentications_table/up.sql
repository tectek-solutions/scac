-- Your SQL goes here
CREATE TABLE authentications (
    id SERIAL PRIMARY KEY, -- Unique ID
    name VARCHAR(32) UNIQUE NOT NULL, -- Name of the authentication service (e.g., Google, Microsoft)
    authentication_url TEXT NOT NULL, -- OAuth2 authorization URL
    refresh_token_url TEXT NOT NULL, -- Refresh token URL
    client_id TEXT NOT NULL, -- OAuth2 client ID
    client_secret TEXT NOT NULL, -- OAuth2 client secret
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP, -- Creation timestamp
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP -- Last update timestamp
);
