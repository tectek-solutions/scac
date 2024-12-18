# SCAC 

## Architecture

- users-service

  Handles user creation, authentication, and authorization.

- authentication-service

  Handles authentication like Google OAuth, Microsoft, ..

- user-tokens-service

  Handles user tokens for authentication.

- apis-service

  Handles APIs like Gmail, Google Drive, Microsoft OneDrive, Outlook, ..

- actions-service

  Handles actions like receiving emails, sending emails, uploading files, ..

- reactions-service

  Handles reactions like sending emails, creating files, ..

- workflows-service

  Handles workflows like sending an email when a new file is uploaded, ..

- triggers-service

  Handles triggers like new email received, new file uploaded, ..

## Adding features

- Adding a new authentication

You just to seed the database with the new authentication service data, like the client ID, client secret, and the redirect URL, ...

- Adding a new API

You just to seed the database with the new API service data, like the name, description, and the authentication service ID, ...

- Adding a new action

You just to seed the database with the new action data, like the name, description, and the API service ID, ...

- Adding a new reaction

You just to seed the database with the new reaction data, like the name, description, and the API service ID, ...
