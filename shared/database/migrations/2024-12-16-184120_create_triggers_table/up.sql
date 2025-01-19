-- Your SQL goes here
CREATE TABLE triggers (
    id SERIAL PRIMARY KEY,
    -- Unique ID
    workflows_id INT NOT NULL,
    -- References the workflow
    status TEXT NOT NULL,
    -- Trigger data
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    -- Creation timestamp
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    -- Last update timestamp
    FOREIGN KEY (workflows_id) REFERENCES workflows (id)
);