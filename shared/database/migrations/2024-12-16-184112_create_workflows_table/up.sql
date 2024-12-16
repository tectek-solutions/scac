-- Your SQL goes here
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
