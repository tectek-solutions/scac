-- Your SQL goes here
CREATE TABLE user_tokens (
    id SERIAL PRIMARY KEY,
    -- Unique ID
    users_id INT NOT NULL,
    -- References the user
    authentications_id INT NOT NULL,
    -- References the authentication service
    access_token TEXT NOT NULL,
    -- OAuth2 access token
    refresh_token TEXT,
    -- Optional refresh token
    expires_at TIMESTAMP NOT NULL,
    -- Token expiration timestamp
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    -- Creation timestamp
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    -- Last update timestamp
    FOREIGN KEY (users_id) REFERENCES users (id),
    FOREIGN KEY (authentications_id) REFERENCES authentications (id)
);