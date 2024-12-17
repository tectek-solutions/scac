-- Your SQL goes here
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
    trigger_date_json_path TEXT NOT NULL, -- JSON path to the trigger data
    trigger_date_format TEXT NOT NULL, -- Conversion string for datetime
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP, -- Creation timestamp
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP, -- Last update timestamp
    FOREIGN KEY (api_id) REFERENCES apis (id)
);
