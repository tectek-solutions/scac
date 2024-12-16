# API Documentation

## **Users API**

### **Base Path**: `/users`

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

---

## **Authentications Services API**

### **Base Path**: `/authentications`

- **GET** `/`  
  - **Access**: Public  
  - **Responses**:  
    - `200 OK`: List of services retrieved.  
    - `500 Internal Server Error`: Error fetching services.

- **GET** `/:id`  
  - **Access**: Public  
  - **Responses**:  
    - `200 OK`: Service details retrieved.  
    - `404 Not Found`: Service ID not found.  
    - `500 Internal Server Error`: Error fetching details.

---

## **User Tokens API**

### **Base Path**: `/user-tokens`

- **POST** `/`  
  - **Access**: Authenticated User  
  - **Responses**:  
    - `201 Created`: Token added successfully.  
    - `400 Bad Request`: Invalid input.  
    - `500 Internal Server Error`: Error creating token.

- **GET** `/users/:id`
  - **Access**: Authenticated User (if `id` matches the authenticated user)
  - **Responses**:  
    - `200 OK`: Tokens retrieved.  
    - `404 Not Found`: User ID not found.  
    - `500 Internal Server Error`: Error fetching tokens.

- **PATCH** `/:id`  
  - **Access**: Authenticated User (if `user_id` matches the authenticated user)
  - **Responses**:  
    - `200 OK`: Token updated successfully.  
    - `400 Bad Request`: Invalid input.  
    - `404 Not Found`: Token ID not found.  
    - `500 Internal Server Error`: Error during update.

- **PUT** `/:id`  
  - **Access**: Authenticated User (if `user_id` matches the authenticated user)
  - **Responses**:  
    - `200 OK`: Token updated successfully.  
    - `400 Bad Request`: Invalid input.  
    - `404 Not Found`: Token ID not found.  
    - `500 Internal Server Error`: Error during update.

- **DELETE** `/:id`  
  - **Access**: Authenticated User  
  - **Responses**:  
    - `204 No Content`: Token deleted.  
    - `404 Not Found`: Token ID not found.  
    - `500 Internal Server Error`: Error during deletion.

---

## **API Services API**

### **Base Path**: `/api-services`

- **GET** `/:id`
  - **Access**: Public  
  - **Responses**:  
    - `200 OK`: API service retrieved.  
    - `404 Not Found`: Service ID not found.  
    - `500 Internal Server Error`: Error fetching service.

- **GET** `/authentication/:id`
  - **Access**: Public  
  - **Responses**:  
    - `200 OK`: List of authentication services retrieved.
    - `404 Not Found`: Service ID not found.
    - `500 Internal Server Error`: Error fetching services.

---

## **Actions API**

### **Base Path**: `/actions`

- **GET** `/:id`
  - **Access**: Public  
  - **Responses**:  
    - `200 OK`: Action retrieved.  
    - `404 Not Found`: Action ID not found.  
    - `500 Internal Server Error`: Error fetching action.

- **GET** `/api-services/:id`
  - **Access**: Public  
  - **Responses**:  
    - `200 OK`: List of actions retrieved.  
    - `500 Internal Server Error`: Error fetching actions.

---

## **Reactions API**

### **Base Path**: `/reactions`

- **GET** `/`  
  - **Access**: Public  
  - **Responses**:  
    - `200 OK`: List of reactions retrieved.  
    - `500 Internal Server Error`: Error fetching reactions.

- **GET** `/:id`
  - **Access**: Public  
  - **Responses**:  
    - `200 OK`: Reaction retrieved.  
    - `404 Not Found`: Reaction ID not found.  
    - `500 Internal Server Error`: Error fetching reaction.


## **Workflows API**

### **Base Path**: `/workflows`

- **POST** `/`  
  - **Access**:  Authenticated User  
  - **Responses**:  
    - `201 Created`: Workflow created successfully.  
    - `400 Bad Request`: Invalid input.  
    - `500 Internal Server Error`: Error creating workflow.

- **GET** `/users/:id`  
  - **Access**: Authenticated User (if `user_id` matches the authenticated user)  
  - **Responses**:  
    - `200 OK`: Workflows retrieved.  
    - `404 Not Found`: User ID not found.  
    - `500 Internal Server Error`: Error fetching workflows.

- **GET** `/:id`  
  - **Access**: Authenticated User
  - **Responses**:  
    - `200 OK`: Workflow retrieved.  
    - `404 Not Found`: Workflow ID not found.  
    - `500 Internal Server Error`: Error fetching workflow.

## **Trigger API**

- **GET** `/users/:id`  
  - **Access**: Authenticated User (if `user_id` matches the authenticated user)  
  - **Responses**:  
    - `200 OK`: Triggers retrieved.  
    - `404 Not Found`: User ID not found.  
    - `500 Internal Server Error`: Error fetching triggers.

- **GET** `/:id`
  - **Access**: Authenticated User
  - **Responses**:  
    - `200 OK`: Trigger retrieved.  
    - `404 Not Found`: Trigger ID not found.  
    - `500 Internal Server Error`: Error fetching trigger.  

---
