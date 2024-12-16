-- Users Table

CREATE TABLE users (
    id SERIAL PRIMARY KEY, -- Unique ID
    name VARCHAR(32) UNIQUE NOT NULL, -- Unique name
    email VARCHAR(128) UNIQUE NOT NULL, -- Email
    password_hash VARCHAR(32) NOT NULL, -- Hashed password
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP, -- Creation timestamp
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP -- Last update timestamp
);

-- Authentication Services Table

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

-- User Tokens Table

CREATE TABLE user_tokens (
    id SERIAL PRIMARY KEY, -- Unique ID
    users_id INT NOT NULL, -- References the user
    authentication_id INT NOT NULL, -- References the authentication service
    access_token TEXT NOT NULL, -- OAuth2 access token
    refresh_token TEXT, -- Optional refresh token
    expires_at TIMESTAMP NOT NULL, -- Token expiration timestamp
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP, -- Creation timestamp
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP, -- Last update timestamp
    FOREIGN KEY (users_id) REFERENCES users (id),
    FOREIGN KEY (authentication_id) REFERENCES authentications (id)
);

-- API Services Table

CREATE TABLE apis (
    id SERIAL PRIMARY KEY, -- Unique ID
    authentication_id INT NOT NULL, -- Reference to the authentication service
    name VARCHAR(32) UNIQUE NOT NULL, -- Name of the API service (e.g., Google Calendar, Outlook)
    base_url TEXT NOT NULL, -- Base URL of the API
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP, -- Creation timestamp
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP, -- Last update timestamp
    FOREIGN KEY (authentication_id) REFERENCES authentications (id) -- Foreign key reference
);

-- API Services Actions Table

CREATE TABLE actions (
    id SERIAL PRIMARY KEY, -- Unique ID
    api_id INT NOT NULL, -- References the API service
    name VARCHAR(32) NOT NULL, -- Unique name
    description TEXT, -- Description
    http_method VARCHAR(8) NOT NULL, -- HTTP method
    http_endpoint TEXT NOT NULL, -- API endpoint
    http_parameters JSONB, -- Parameters
    http_headers JSONB, -- Headers
    http_body JSONB, -- Body
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP, -- Creation timestamp
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP, -- Last update timestamp
    FOREIGN KEY (api_id) REFERENCES apis (id)
);

-- API Services Reactions Table

CREATE TABLE reactions (
    id SERIAL PRIMARY KEY, -- Unique ID
    api_id INT NOT NULL, -- References the API service
    name VARCHAR(32) NOT NULL, -- Unique name
    description TEXT, -- Description
    http_method VARCHAR(8) NOT NULL, -- HTTP method
    http_endpoint TEXT NOT NULL, -- API endpoint
    http_parameters JSONB, -- Parameters
    http_headers JSONB, -- Headers
    http_body JSONB, -- Body
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP, -- Creation timestamp
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP, -- Last update timestamp
    FOREIGN KEY (api_id) REFERENCES apis (id)
);

-- Workflows Table

CREATE TABLE workflows (
    id SERIAL PRIMARY KEY,  -- Unique ID
    user_id INT NOT NULL, -- References the user
    name VARCHAR(32) NOT NULL, -- Workflow name
    description TEXT, -- Workflow description
    action_id INT NOT NULL, -- References the trigger action
    reaction_id INT NOT NULL, -- References the reaction action
    data_transformation JSONB, -- Transformation rules for data
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP, -- Creation timestamp
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP, -- Last update timestamp
    FOREIGN KEY (user_id) REFERENCES users (id),
    FOREIGN KEY (action_id) REFERENCES actions (id),
    FOREIGN KEY (reaction_id) REFERENCES reactions (id)
);

-- Triggers Table

CREATE TABLE triggers (
    id SERIAL PRIMARY KEY, -- Unique ID
    workflow_id INT NOT NULL, -- References the workflow
    data JSONB, -- Trigger data
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP, -- Creation timestamp
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP, -- Last update timestamp
    FOREIGN KEY (workflow_id) REFERENCES workflows (id)
);
