-- Your SQL goes here
CREATE TABLE reactions (
    id SERIAL PRIMARY KEY,
    api_service_id INT NOT NULL,
    name VARCHAR(32) NOT NULL,
    description TEXT,
    endpoint TEXT NOT NULL,
    method VARCHAR(10) NOT NULL,
    headers JSONB,
    params JSONB,
    json_path TEXT,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (api_service_id) REFERENCES api_services(id)
);