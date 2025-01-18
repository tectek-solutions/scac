-- Your SQL goes here
CREATE TABLE actions (
    id SERIAL PRIMARY KEY,
    -- Unique ID
    apis_id INT NOT NULL,
    -- References the API service
    name VARCHAR(64) NOT NULL,
    -- Unique name
    description TEXT,
    -- Description
    http_method VARCHAR(8) NOT NULL,
    -- HTTP method
    http_endpoint TEXT NOT NULL,
    -- API endpoint
    http_parameters JSONB,
    -- Parameters
    http_headers JSONB,
    -- Headers
    http_body JSONB,
    -- Body
    data_keys JSONB,
    -- Data keys
    last_id_json_path TEXT,
    -- Last JSON path
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    -- Creation timestamp
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    -- Last update timestamp
    FOREIGN KEY (apis_id) REFERENCES apis (id)
);