-- Your SQL goes here
CREATE TABLE workflows (
    id SERIAL PRIMARY KEY,
    -- Unique ID
    users_id INT NOT NULL,
    -- References the user
    name VARCHAR(64) NOT NULL,
    -- Workflow name
    description TEXT,
    -- Workflow description
    actions_id INT NOT NULL,
    -- References the trigger action
    reactions_id INT NOT NULL,
    -- References the reaction action
    action_data JSONB,
    -- Data for the trigger action
    reaction_data JSONB,
    -- Data for the reaction action
    last_id TEXT,
    -- Last hash of the workflow
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    -- Creation timestamp
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    -- Last update timestamp
    FOREIGN KEY (users_id) REFERENCES users (id),
    FOREIGN KEY (actions_id) REFERENCES actions (id),
    FOREIGN KEY (reactions_id) REFERENCES reactions (id)
);
