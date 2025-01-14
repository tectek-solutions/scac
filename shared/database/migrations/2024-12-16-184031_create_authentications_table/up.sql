-- Your SQL goes here
CREATE TABLE authentications (
    id SERIAL PRIMARY KEY,
    -- Unique ID
    name VARCHAR(32) UNIQUE NOT NULL,
    -- Name of the authentication service (e.g., Google, Microsoft)
    authorization_url TEXT NOT NULL,
    -- OAuth2 authorization URL
    authorization_http_parameters JSONB,
    -- OAuth2 authorization URL parameters
    token_url TEXT NOT NULL,
    -- Token URL
    token_url_http_parameters
    -- Token URL parameters
    client_id TEXT NOT NULL,
    -- OAuth2 client ID
    client_secret TEXT NOT NULL,
    -- OAuth2 client secret
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    -- Creation timestamp
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP -- Last update timestamp
);