-- Your SQL goes here
CREATE TABLE user_tokens (
    id SERIAL PRIMARY KEY, -- Unique ID
    users_id INT NOT NULL, -- References the user
    authentication_id INT NOT NULL, -- References the authentication service
    access_token TEXT NOT NULL, -- OAuth2 access token
    access_token_expires_at TIMESTAMP NOT NULL, -- Access token expiration timestamp
    refresh_token TEXT NOT NULL, -- Optional refresh token
    refresh_token_expires_at TIMESTAMP NOT NULL, -- Optional refresh token expiration timestamp
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP, -- Creation timestamp
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP, -- Last update timestamp
    FOREIGN KEY (users_id) REFERENCES users (id),
    FOREIGN KEY (authentication_id) REFERENCES authentications (id)
);
