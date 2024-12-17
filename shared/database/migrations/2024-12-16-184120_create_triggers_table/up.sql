-- Your SQL goes here
CREATE TABLE triggers (
    id SERIAL PRIMARY KEY, -- Unique ID
    workflow_id INT NOT NULL, -- References the workflow
    data JSONB, -- Trigger data
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP, -- Creation timestamp
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP, -- Last update timestamp
    FOREIGN KEY (workflow_id) REFERENCES workflows (id)
);
