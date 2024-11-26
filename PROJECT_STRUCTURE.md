# **Microservices Architecture with Unified Flutter Client**

The microservices architecture involves splitting an application into small, independently deployable services, each responsible for a specific domain. For the automation platform, this approach enables scalability and maintainability by focusing on modular design. A single **Flutter client** is now used to serve both web and mobile platforms, simplifying client development and deployment.

---

## **Microservices Overview**

### **1. User Management Service**

- **Responsibility**: Handles user registration, authentication, and profile management.
- **API Endpoints**:
  - `POST /users/register`: Register a new user.
  - `POST /users/login`: Authenticate user credentials.
  - `GET /users/{id}`: Retrieve user profile details.

### **2. Service Integration Service**

- **Responsibility**: Manages external services (e.g., Google, Facebook, OneDrive) users can connect to.
- **API Endpoints**:
  - `GET /services`: List all available services.
  - `POST /services/{service_id}/subscribe`: Connect a user to a service using OAuth2.

### **3. Action and Reaction Service**

- **Responsibility**: Manages actions (e.g., "new email received") and reactions (e.g., "save to OneDrive").
- **API Endpoints**:
  - `GET /actions`: Retrieve a list of available actions.
  - `GET /reactions`: Retrieve a list of available reactions.
  - `POST /actions/{action_id}/trigger`: Trigger an action based on conditions.

### **4. Workflow (SCAC) Service**

- **Responsibility**: Handles automation workflows combining actions and reactions.
- **API Endpoints**:
  - `POST /workflows`: Create a new workflow.
  - `GET /workflows/{workflow_id}`: Retrieve a specific workflow.
  - `GET /workflows`: List all user-created workflows.

### **5. Trigger Service**

- **Responsibility**: Monitors events to execute workflows.
- **API Endpoints**:
  - `POST /triggers/{trigger_id}/start`: Start monitoring for a specific event.
- **Technologies**:
  - Event-driven frameworks (e.g., Node.js, Go).
  - Messaging systems like Redis or RabbitMQ for queue management.

### **6. Unified Flutter Client**

- **Responsibility**: Serves as a unified client for both web and mobile platforms.
- **Interaction**: Interfaces with backend microservices via REST APIs for seamless integration.
- **Technologies**: Flutter for a consistent UI and shared codebase.

---

## **Microservices Communication**

- **API Gateway**: Routes client requests to appropriate microservices (e.g., Kong, Nginx).
- **Message Broker**: Asynchronous communication via RabbitMQ or Kafka for event-driven workflows.
- **Protocol**: RESTful APIs for synchronous communication or gRPC for high-performance needs.

---

## **Monorepo Structure**

To streamline development, all services and the client are stored in a single **monorepo**.

### **Monorepo Organization**

```
/automation-platform
│
├── /services
│   ├── /user-management-service
│   ├── /service-integration-service
│   ├── /action-reaction-service
│   ├── /workflow-service
│   └── /trigger-service
│
├── /client
│   └── /flutter-app
│
├── /shared
│   ├── /utils (shared utilities like authentication and validation)
│   ├── /models (common data models, e.g., User, Service)
│   └── /config (environment variables and shared configurations)
│
├── docker-compose.yml
├── README.md
└── /docs (API and architecture documentation)
```

### **Structure Details**

1. **`/services`**:
   - Contains business logic, API endpoints, and tests for each microservice.
   - Includes a Dockerfile for containerization.
   - The **API Gateway** ensures secure and efficient routing.

2. **`/client`**:
   - Houses the Flutter project, supporting both mobile and web from a unified codebase.
   - Benefits from shared business logic and responsive UI components.

3. **`/shared`**:
   - **`/utils`**: Shared functions (e.g., logging, authentication helpers).
   - **`/models`**: Common data models for consistency across services.
   - **`/config`**: Centralized configuration management.

4. **`docker-compose.yml`**:
   - Defines how services and the Flutter client are orchestrated, enabling consistent deployment.

5. **`README.md`**:
   - Provides project setup instructions and service descriptions.

---

## **Advantages of This Architecture**

### **Microservices**

- **Scalability**: Independently deploy and scale each service.
- **Flexibility**: Modify or replace services without affecting the entire platform.
- **Domain Separation**: Each service focuses on a specific functionality.

### **Unified Flutter Client**

- **Efficiency**: Single codebase for web and mobile reduces development time.
- **Consistency**: Unified design and behavior across platforms.
- **Simplified Maintenance**: Shared bug fixes and features between web and mobile.

### **Monorepo**

- **Streamlined Collaboration**: Centralized repository facilitates teamwork.
- **Code Reusability**: Shared utilities and models reduce redundancy.
- **Centralized Configuration**: Simplifies dependency and version management.

---

## **Final Architecture Overview**

1. **Backend**: Microservices for user management, integrations, actions, workflows, and triggers.
2. **Frontend**: A single Flutter client for web and mobile.
3. **Deployment**: Docker Compose for orchestrating services and clients.
4. **Communication**: API Gateway and message brokers for seamless service interactions.

This architecture supports scalability, simplifies cross-platform development, and enhances maintainability while ensuring modularity and separation of concerns.
