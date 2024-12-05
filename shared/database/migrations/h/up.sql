-- Your SQL goes here
CREATE TABLE triggers (
    id SERIAL PRIMARY KEY,
    workflow_id INT NOT NULL,
    data JSONB NOT NULL,
    status status_enum NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (workflow_id) REFERENCES workflows(id)
);