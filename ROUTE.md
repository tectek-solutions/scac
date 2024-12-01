# API Documentation

## **Users API**

### **Base Path**: `/api/users`

- **POST** `/register`  
  - **Access**: Public  
  - **Responses**:  
    - `201 Created`: User successfully registered.  
    - `400 Bad Request`: Missing fields or duplicate username/email.  
    - `500 Internal Server Error`: Error during registration.

- **POST** `/login`  
  - **Access**: Public  
  - **Responses**:  
    - `200 OK`: Authentication successful, returns JWT.  
    - `400 Bad Request`: Missing credentials.  
    - `401 Unauthorized`: Incorrect email or password.  
    - `500 Internal Server Error`: Error during authentication.

- **GET** `/me`  
  - **Access**: Authenticated User  
  - **Responses**:  
    - `200 OK`: User profile retrieved.  
    - `401 Unauthorized`: Token missing or invalid.  
    - `500 Internal Server Error`: Error retrieving profile.

- **GET** `/refresh-token`  
  - **Access**: Authenticated User  
  - **Responses**:  
    - `200 OK`: New JWT issued.  
    - `401 Unauthorized`: Token missing or invalid.  
    - `500 Internal Server Error`: Error generating token.

- **GET** `/:id`  
  - **Access**: Admin  
  - **Responses**:  
    - `200 OK`: User profile retrieved.  
    - `404 Not Found`: User ID not found.  
    - `500 Internal Server Error`: Database/server error.

- **PUT** `/:id`  
  - **Access**: Admin or Authenticated User (if `id` matches the authenticated user)  
  - **Responses**:  
    - `200 OK`: User details updated.  
    - `400 Bad Request`: Invalid input (e.g., invalid email format).  
    - `404 Not Found`: User ID not found.  
    - `500 Internal Server Error`: Server error during update.

- **DELETE** `/:id`  
  - **Access**: Admin or Authenticated User (if `id` matches the authenticated user)  
  - **Responses**:  
    - `204 No Content`: User deleted.  
    - `404 Not Found`: User ID not found.  
    - `500 Internal Server Error`: Error during deletion.

---

## **Authentication Services API**

### **Base Path**: `/api/authentification-services`

- **GET** `/`  
  - **Access**: Public  
  - **Responses**:  
    - `200 OK`: List of services retrieved.  
    - `500 Internal Server Error`: Error fetching services.

- **POST** `/`  
  - **Access**: Admin  
  - **Responses**:  
    - `201 Created`: Service added.  
    - `400 Bad Request`: Invalid input or duplicate name.  
    - `500 Internal Server Error`: Server error.

- **GET** `/:id`  
  - **Access**: Public  
  - **Responses**:  
    - `200 OK`: Service details retrieved.  
    - `404 Not Found`: Service ID not found.  
    - `500 Internal Server Error`: Error fetching details.

- **PUT** `/:id`  
  - **Access**: Admin  
  - **Responses**:  
    - `200 OK`: Service updated.  
    - `400 Bad Request`: Invalid input.  
    - `404 Not Found`: Service ID not found.  
    - `500 Internal Server Error`: Error during update.

- **DELETE** `/:id`  
  - **Access**: Admin  
  - **Responses**:  
    - `204 No Content`: Service deleted.  
    - `404 Not Found`: Service ID not found.  
    - `500 Internal Server Error`: Error during deletion.

- **GET** `/:id/api-services`  
  - **Access**: Admin  
  - **Responses**:  
    - `200 OK`: List of API services retrieved.  
    - `404 Not Found`: Service ID not found.  
    - `500 Internal Server Error`: Error fetching services.

- **GET** `/:id/tokens`  
  - **Access**: Admin or Authenticated User (with access to the service)  
  - **Responses**:  
    - `200 OK`: Tokens retrieved.  
    - `404 Not Found`: Service ID not found.  
    - `500 Internal Server Error`: Error fetching tokens.

---

## **User Tokens API**

### **Base Path**: `/api/user-tokens`

- **POST** `/`  
  - **Access**: Admin or Authenticated User  
  - **Responses**:  
    - `201 Created`: Token added successfully.  
    - `400 Bad Request`: Invalid input.  
    - `500 Internal Server Error`: Error creating token.

- **GET** `/user/:userId`  
  - **Access**: Admin or Authenticated User (if `userId` matches the authenticated user)  
  - **Responses**:  
    - `200 OK`: Tokens retrieved.  
    - `404 Not Found`: User ID not found.  
    - `500 Internal Server Error`: Error fetching tokens.

- **PUT** `/:id`  
  - **Access**: Admin or Authenticated User  
  - **Responses**:  
    - `200 OK`: Token updated successfully.  
    - `400 Bad Request`: Invalid input.  
    - `404 Not Found`: Token ID not found.  
    - `500 Internal Server Error`: Error during update.

- **DELETE** `/:id`  
  - **Access**: Admin or Authenticated User  
  - **Responses**:  
    - `204 No Content`: Token deleted.  
    - `404 Not Found`: Token ID not found.  
    - `500 Internal Server Error`: Error during deletion.

---

## **API Services API**

### **Base Path**: `/api/api-services`

- **POST** `/`  
  - **Access**: Admin  
  - **Responses**:  
    - `201 Created`: API service added.  
    - `400 Bad Request`: Invalid input or duplicate name.  
    - `500 Internal Server Error`: Error adding service.

- **GET** `/:id`
  - **Access**: Public  
  - **Responses**:  
    - `200 OK`: API service retrieved.  
    - `404 Not Found`: Service ID not found.  
    - `500 Internal Server Error`: Error fetching service.

- **PUT** `/:id`
  - **Access**: Admin  
  - **Responses**:  
    - `200 OK`: API service updated.  
    - `400 Bad Request`: Invalid input.  
    - `404 Not Found`: Service ID not found.  
    - `500 Internal Server Error`: Error updating service.

- **DELETE** `/:id`
  - **Access**: Admin  
  - **Responses**:  
    - `204 No Content`: API service deleted.  
    - `404 Not Found`: Service ID not found.  
    - `500 Internal Server Error`: Error deleting service.

---

## **Actions API**

### **Base Path**: `/api/actions`

- **GET** `/`  
  - **Access**: Public  
  - **Responses**:  
    - `200 OK`: List of actions retrieved.  
    - `500 Internal Server Error`: Error fetching actions.

- **POST** `/`
  - **Access**: Admin  
  - **Responses**:  
    - `201 Created`: Action added.  
    - `400 Bad Request`: Invalid input or duplicate name.  
    - `500 Internal Server Error`: Error adding action.

- **GET** `/:id`
  - **Access**: Public  
  - **Responses**:  
    - `200 OK`: Action retrieved.  
    - `404 Not Found`: Action ID not found.  
    - `500 Internal Server Error`: Error fetching action.

- **PUT** `/:id`
  - **Access**: Admin  
  - **Responses**:  
    - `200 OK`: Action updated.  
    - `400 Bad Request`: Invalid input.  
    - `404 Not Found`: Action ID not found.  
    - `500 Internal Server Error`: Error updating action.

- **DELETE** `/:id`
  - **Access**: Admin  
  - **Responses**:  
    - `204 No Content`: Action deleted.  
    - `404 Not Found`: Action ID not found.  
    - `500 Internal Server Error`: Error deleting action.

---

## **Reactions API**

### **Base Path**: `/api/reactions`

- **GET** `/`  
  - **Access**: Public  
  - **Responses**:  
    - `200 OK`: List of reactions retrieved.  
    - `500 Internal Server Error`: Error fetching reactions.

- **POST** `/`
  - **Access**: Admin  
  - **Responses**:  
    - `201 Created`: Reaction added.  
    - `400 Bad Request`: Invalid input or duplicate name.  
    - `500 Internal Server Error`: Error adding reaction.

- **GET** `/:id`
  - **Access**: Public  
  - **Responses**:  
    - `200 OK`: Reaction retrieved.  
    - `404 Not Found`: Reaction ID not found.  
    - `500 Internal Server Error`: Error fetching reaction.


## **Workflows API**

### **Base Path**: `/api/workflows`

- **POST** `/`  
  - **Access**: Admin or Authenticated User  
  - **Responses**:  
    - `201 Created`: Workflow created successfully.  
    - `400 Bad Request`: Invalid input.  
    - `500 Internal Server Error`: Error creating workflow.

- **GET** `/user/:userId`  
  - **Access**: Admin or Authenticated User (if `userId` matches the authenticated user)  
  - **Responses**:  
    - `200 OK`: Workflows retrieved successfully.  
    - `404 Not Found`: User ID not found.  
    - `500 Internal Server Error`: Error fetching workflows.

- **GET** `/:id`  
  - **Access**: Admin or Authenticated User (with access to the workflow)  
  - **Responses**:  
    - `200 OK`: Workflow retrieved.  
    - `404 Not Found`: Workflow ID not found.  
    - `500 Internal Server Error`: Error fetching workflow.

- **PUT** `/:id`  
  - **Access**: Admin or Authenticated User  
  - **Responses**:  
    - `200 OK`: Workflow updated successfully.  
    - `400 Bad Request`: Invalid input.  
    - `404 Not Found`: Workflow ID not found.  
    - `500 Internal Server Error`: Error during update.

- **DELETE** `/:id`  
  - **Access**: Admin or Authenticated User  
  - **Responses**:  
    - `204 No Content`: Workflow deleted.  
    - `404 Not Found`: Workflow ID not found.  
    - `500 Internal Server Error`: Error during deletion.

---

## **Roles Summary**

- **Public**: No token required.  
- **Authenticated User**: Requires valid JWT.  
- **Admin**: Requires admin JWT.  
- **Service**: Requires service token.  
