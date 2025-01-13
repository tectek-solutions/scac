-- Your SQL goes here
CREATE TABLE apis (
    id SERIAL PRIMARY KEY,
    -- Unique ID
    authentications_id INT NOT NULL,
    -- Reference to the authentication service
    name VARCHAR(32) UNIQUE NOT NULL,
    -- Name of the API service (e.g., Google Calendar, Outlook)
    base_url TEXT NOT NULL,
    -- Base URL of the API
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    -- Creation timestamp
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    -- Last update timestamp
    FOREIGN KEY (authentications_id) REFERENCES authentications (id) -- Foreign key reference
);