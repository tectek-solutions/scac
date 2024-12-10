-- Your SQL goes here
CREATE TABLE user_tokens (
    id SERIAL PRIMARY KEY,
    user_id INT NOT NULL,
    auth_service_id INT NOT NULL,
    access_token TEXT NOT NULL,
    refresh_token TEXT,
    expires_at TIMESTAMP NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (user_id) REFERENCES users(id),
    FOREIGN KEY (auth_service_id) REFERENCES authentification(id)
);