# SCAC 

## Core of the Project

SCAC (Service Coordination and Automation Center) is designed to streamline and automate the interaction between various services and APIs. The project's core revolves around handling user authentication, API interactions, automated actions, and reactions based on specific triggers, all orchestrated through workflows.

### Main Components

- **users-service**: 
  Handles user creation, authentication, and authorization.

- **authentication-service**: 
  Manages authentication methods like Google OAuth, Microsoft, etc.

- **user-tokens-service**: 
  Handles user tokens for authentication purposes.

- **apis-service**: 
  Manages API interactions with services like Gmail, Google Drive, Microsoft OneDrive, Outlook, etc.

- **actions-service**: 
  Manages actions such as receiving emails, sending emails, uploading files, etc.

- **reactions-service**: 
  Manages reactions like sending emails or creating files in response to actions.

- **workflows-service**: 
  Coordinates workflows such as sending an email when a new file is uploaded.

- **triggers-service**: 
  Handles triggers such as a new email received or a new file uploaded.

### Adding Features

- **Adding a new authentication**: 
  Seed the database with the new authentication service data, including the client ID, client secret, and the redirect URL.

- **Adding a new API**: 
  Seed the database with the new API service data, including the name, description, and the authentication service ID.

- **Adding a new action**: 
  Seed the database with the new action data, including the name, description, and the API service ID.

- **Adding a new reaction**: 
  Seed the database with the new reaction data, including the name, description, and the API service ID.
