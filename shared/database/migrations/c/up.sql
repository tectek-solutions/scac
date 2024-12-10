-- Your SQL goes here
CREATE TABLE api_services (
    id SERIAL PRIMARY KEY,
    auth_service_id INT NOT NULL,
    name VARCHAR(32) UNIQUE NOT NULL,
    base_url  TEXT NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (auth_service_id) REFERENCES authentification(id)
);