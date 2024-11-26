# Database Schema

## **SQL Database (User Management with Multiple Tokens per Service)**

```sql
CREATE TABLE Users (
    id SERIAL PRIMARY KEY,             -- Unique user ID
    username VARCHAR(50) UNIQUE NOT NULL, -- Unique username
    email VARCHAR(100) UNIQUE NOT NULL,   -- User email
    password_hash TEXT NOT NULL,       -- Hashed password
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP, -- Account creation date
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP
);

CREATE TABLE Services (
    id SERIAL PRIMARY KEY,             -- Unique service ID
    name VARCHAR(50) UNIQUE NOT NULL,  -- Service name (e.g., GitHub, Gmail)
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP
);

CREATE TABLE User_Tokens (
    id SERIAL PRIMARY KEY,             -- Unique token ID
    user_id INT NOT NULL,              -- References the user
    service_id INT NOT NULL,           -- References the service
    token TEXT NOT NULL,               -- OAuth2 access token
    refresh_token TEXT,                -- Optional refresh token
    expires_at TIMESTAMP,              -- Expiration time of the token
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    FOREIGN KEY (user_id) REFERENCES Users (id),
    FOREIGN KEY (service_id) REFERENCES Services (id)
);

-- Indexes for faster queries
CREATE INDEX idx_users_email ON Users (email);
CREATE INDEX idx_services_name ON Services (name);
CREATE INDEX idx_user_tokens_user_id ON User_Tokens (user_id);
CREATE INDEX idx_user_tokens_service_id ON User_Tokens (service_id);
```

## **NoSQL Database (Dynamic Service Data)**

1. **Services Collection**

   ```json
   {
       "_id": "service_id_1",         // Unique ID for the service
       "name": "GitHub",              // Service name
       "actions": [                   // List of supported actions
           { "id": "action_1", "name": "New Commit" },
           { "id": "action_2", "name": "New Pull Request" }
       ],
       "reactions": [                 // List of supported reactions
           { "id": "reaction_1", "name": "Create Issue" },
           { "id": "reaction_2", "name": "Comment on Issue" }
       ]
   }
   ```

2. **Workflows Collection**

   ```json
   {
       "_id": "workflow_id_1",       // Unique ID for the workflow
       "user_id": "user_id_1",       // References the user
       "name": "GitHub to Slack",    // Workflow name
       "actions": [                  // List of actions
           {
               "id": "action_1",
               "service": "GitHub",
               "params": { "repo": "my-repo" },
               "token_id": "token_id_123"  // References the token in SQL
           }
       ],
       "reactions": [                // List of reactions
           {
               "id": "reaction_1",
               "service": "Slack",
               "params": { "channel": "#general" },
               "token_id": "token_id_456"  // References the token in SQL
           }
       ],
       "conditions": {               // Conditional logic
           "if": "action_1.status == 'success'"
       },
       "created_at": "2024-11-25T12:00:00Z", // Timestamp
       "updated_at": "2024-11-25T13:00:00Z"
   }
   ```

3. **Trigger Logs Collection**

   ```json
   {
       "_id": "log_id_1",            // Unique ID for the log
       "workflow_id": "workflow_id_1", // Workflow that triggered the log
       "trigger_event": {            // Trigger details
           "service": "GitHub",
           "action": "New Commit",
           "timestamp": "2024-11-25T12:15:00Z"
       },
       "reaction_results": [         // Reactions executed
           { "reaction": "Create Issue", "status": "success" }
       ],
       "status": "success",          // Overall status
       "log_timestamp": "2024-11-25T12:20:00Z"
   }
   ```

---

## **Integration and Usage**

- **SQL Database**: Ensures that each user can have multiple tokens for different services. Tokens are stored securely, and their relationships are managed via foreign keys.
- **NoSQL Database**: Enables dynamic workflows and service configurations by linking actions and reactions to the appropriate tokens (via `token_id`).
