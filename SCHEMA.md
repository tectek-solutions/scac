
# **Database Schema Design**

## **users Table**

The base table for user management.

```sql
CREATE TABLE users (
    id SERIAL PRIMARY KEY, -- Unique user ID
    username VARCHAR(32) UNIQUE NOT NULL, -- Unique username
    email VARCHAR(128) UNIQUE NOT NULL, -- User email
    password_hash BIT(256) NOT NULL, -- Hashed password
    role user_role_enum NOT NULL, -- User role
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP, -- Creation timestamp
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP -- Last update timestamp
);
```

---

## **authentication_services Table**

Table to store details of external authentication providers like Google and Microsoft.

```sql
CREATE TABLE authentication_services (
    id SERIAL PRIMARY KEY, -- Unique ID for the authentication service
    name VARCHAR(32) UNIQUE NOT NULL, -- Name of the authentication service (e.g., Google, Microsoft)
    auth_url TEXT NOT NULL, -- Authentication URL
    token_url TEXT NOT NULL, -- Token exchange URL
    client_id TEXT NOT NULL, -- OAuth2 client ID
    client_secret TEXT NOT NULL, -- OAuth2 client secret
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP, -- Creation timestamp
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP -- Last update timestamp
);
```

---

## **user_tokens Table**

Table to manage user tokens for authentication services.

```sql
CREATE TABLE user_tokens (
    id SERIAL PRIMARY KEY, -- Unique token ID
    user_id INT NOT NULL, -- References the user
    auth_service_id INT NOT NULL, -- References the authentication service
    access_token TEXT NOT NULL, -- OAuth2 access token
    refresh_token TEXT, -- Optional refresh token
    expires_at TIMESTAMP NOT NULL, -- Token expiration timestamp
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP, -- Creation timestamp
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP, -- Last update timestamp
    FOREIGN KEY (user_id) REFERENCES users (id),
    FOREIGN KEY (auth_service_id) REFERENCES authentication_services (id)
);
```

---

## **api_services Table**

Table to store details of APIs provided by authentication services.

```sql
CREATE TABLE api_services (
    id SERIAL PRIMARY KEY, -- Unique ID for the API service
    auth_service_id INT NOT NULL, -- Reference to the authentication service
    name VARCHAR(32) UNIQUE NOT NULL, -- Name of the API service (e.g., Google Calendar, Outlook)
    base_url TEXT NOT NULL, -- Base URL of the API
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP, -- Creation timestamp
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP, -- Last update timestamp
    FOREIGN KEY (auth_service_id) REFERENCES authentication_services (id) -- Foreign key reference
);
```

## **api_services_action Table**

Table to store triggers for API services.

```sql
CREATE TABLE api_services_actions (
    id SERIAL PRIMARY KEY, -- Unique trigger ID
    api_service_id INT NOT NULL, -- References the API service
    name VARCHAR(32) NOT NULL, -- Trigger name
    description TEXT, -- Trigger description
    endpoint TEXT NOT NULL, -- API endpoint for the trigger
    method http_method_enum NOT NULL, -- HTTP method for the trigger
    headers JSONB, -- Headers for the trigger, stored as JSON
    params JSONB, -- Parameters for the trigger, stored as JSON
    json_path TEXT, -- JSON path for data extraction
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP, -- Creation timestamp
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP, -- Last update timestamp
    FOREIGN KEY (api_service_id) REFERENCES api_services (id)
);
```

## **api_services_reactions Table**

Table to store reactions for API services.

```sql
CREATE TABLE api_services_reactions (
    id SERIAL PRIMARY KEY, -- Unique reaction ID
    api_service_id INT NOT NULL, -- References the API service
    name VARCHAR(32) NOT NULL, -- Reaction name
    description TEXT, -- Reaction description
    endpoint TEXT NOT NULL, -- API endpoint for the reaction
    method http_method_enum NOT NULL, -- HTTP method for the reaction
    headers JSONB, -- Headers for the trigger, stored as JSON
    params JSONB, -- Parameters for the reaction, stored as JSON
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP, -- Creation timestamp
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP, -- Last update timestamp
    FOREIGN KEY (api_service_id) REFERENCES api_services (id)
);
```

---

## **workflows Table**

Table to store workflows created by users.

```sql
CREATE TABLE workflows (
    id SERIAL PRIMARY KEY, -- Unique workflow ID
    user_id INT NOT NULL, -- References the user
    name VARCHAR(32) NOT NULL, -- Workflow
    description TEXT, -- Workflow description
    action_id INT NOT NULL, -- References the trigger action
    reaction_id INT NOT NULL, -- References the reaction action
    data_transformation JSONB, -- Transformation rules for data
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP, -- Creation timestamp
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP, -- Last update timestamp
    FOREIGN KEY (user_id) REFERENCES users (id)
    FOREIGN KEY (action_id) REFERENCES api_services_actions (id)
    FOREIGN KEY (reaction_id) REFERENCES api_services_reactions (id)
);
```

## Enumerated Types

```sql
CREATE TYPE user_role_enum AS ENUM ('service', 'admin', 'user');

```sql
CREATE TYPE http_method_enum AS ENUM ('GET', 'HEAD', 'POST', 'PUT', 'DELETE', 'CONNECT', 'OPTIONS', 'TRACE', 'PATCH');
```

```sql
CREATE TYPE status_enum AS ENUM ('success', 'failure');
```
