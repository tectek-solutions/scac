-- Your SQL goes here
CREATE TABLE workflows (
    id SERIAL PRIMARY KEY,
    user_id INT NOT NULL,
    name VARCHAR(32) UNIQUE NOT NULL,
    description TEXT,
    action_id INT NOT NULL,
    reaction_id INT NOT NULL,
    data_transformation JSONB,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (user_id) REFERENCES users(id),
    FOREIGN KEY (action_id) REFERENCES api_services_actions(id),
    FOREIGN KEY (reaction_id) REFERENCES api_services_reactions(id)
);