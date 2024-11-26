# ExpressJS + Sequelize + SQLite (Todo Application)

This is a simple **To-Do API** built with **Express** and **SQLite** using **Sequelize ORM** for data modeling. The API allows you to perform CRUD (Create, Read, Update, Delete) operations on To-Do items, each with a `name` and an optional `description`.

## Project Structure

```
.
├── index.js         # Main server file
├── models           # Folder for Sequelize models
│   └── todo.js      # Model definition for To-Do items
├── routes           # Folder for Express routes
│   └── todos.js     # To-Do routes
└── db.sqlite        # SQLite database file (auto-generated)
```

### 1. **index.js**: Main server file

This is the entry point of the application. It initializes the server, connects to the database, and handles routing.

### 2. **models/todo.js**: Sequelize model definition

This file defines the schema for the `Todo` model using Sequelize ORM. The model has an `id`, `name`, and `description` field.

### 3. **routes/todos.js**: Express routes for To-Do items

This file contains all the routes for managing To-Do items (`POST`, `GET`, `PUT`, and `DELETE`).

---

## Installation

### Running the App

1. Clone the repository or create the project manually.
2. Navigate to the project directory and install dependencies:

```bash
npm install
```

3. Start the server:

```bash
node index.js
```

The server will start running on `http://localhost:8000`.

---

## API Endpoints

### 1. **POST /todos**

Create a new To-Do item. The `name` field is required, while the `description` field is optional.

**Request body**:

```json
{
  "name": "Buy groceries",
  "description": "Milk, eggs, bread"
}
```

**Example `curl` command**:

```bash
curl -X POST http://localhost:8000/todos \
-H "Content-Type: application/json" \
-d '{"name": "Buy groceries", "description": "Milk, eggs, bread"}'
```

**Response** (201 Created):

```json
{
  "id": 1,
  "name": "Buy groceries",
  "description": "Milk, eggs, bread"
}
```

### 2. **GET /todos**

Retrieve all To-Do items.

**Example `curl` command**:

```bash
curl http://localhost:8000/todos
```

**Response**:

```json
[
  {
    "id": 1,
    "name": "Buy groceries",
    "description": "Milk, eggs, bread"
  },
  {
    "id": 2,
    "name": "Walk the dog",
    "description": "Take the dog for a walk in the park"
  }
]
```

### 3. **GET /todos/:id**

Retrieve a specific To-Do item by its ID.

**Example `curl` command**:

```bash
curl http://localhost:8000/todos/1
```

**Response**:

```json
{
  "id": 1,
  "name": "Buy groceries",
  "description": "Milk, eggs, bread"
}
```

### 4. **PUT /todos/:id**

Update an existing To-Do item by its ID. You can update both the `name` and `description`.

**Request body**:

```json
{
  "name": "Buy groceries",
  "description": "Milk, eggs, bread, and butter"
}
```

**Example `curl` command**:

```bash
curl -X PUT http://localhost:8000/todos/1 \
-H "Content-Type: application/json" \
-d '{"name": "Buy groceries", "description": "Milk, eggs, bread, and butter"}'
```

**Response**:

```json
{
  "id": 1,
  "name": "Buy groceries",
  "description": "Milk, eggs, bread, and butter"
}
```

### 5. **DELETE /todos/:id**

Delete a To-Do item by its ID.

**Example `curl` command**:

```bash
curl -X DELETE http://localhost:8000/todos/1
```

**Response**:

```text
(No content - 204 No Content)
```

---

## Code Explanation

### 1. **index.js**

```javascript
const express = require('express');
const bodyParser = require('body-parser');
const { sequelize } = require('./models/todo');
const todoRoutes = require('./routes/todos');

const app = express();
const PORT = 8000;

app.use(bodyParser.json());

// Use the To-Do routes
app.use('/todos', todoRoutes);

// Initialize database and start server
sequelize.sync({ force: true }).then(() => {
  console.log('Database synced');
  app.listen(PORT, () => {
    console.log(`Server is running on http://localhost:${PORT}`);
  });
});
```

- **`express`**: Express is used to handle HTTP requests.
- **`bodyParser.json()`**: This middleware is used to parse incoming JSON requests.
- **`sequelize.sync({ force: true })`**: This synchronizes the Sequelize model with the database. The `{ force: true }` option will drop and recreate the database every time the server starts (useful for development).
- **`todoRoutes`**: These routes are imported from the `routes/todos.js` file to handle CRUD operations for To-Do items.

---

### 2. **models/todo.js**

```javascript
const { Sequelize, DataTypes } = require('sequelize');
const sequelize = new Sequelize({
  dialect: 'sqlite',
  storage: './db.sqlite',
});

const Todo = sequelize.define('Todo', {
  id: {
    type: DataTypes.INTEGER,
    primaryKey: true,
    autoIncrement: true,
  },
  name: {
    type: DataTypes.STRING,
    allowNull: false,
  },
  description: {
    type: DataTypes.STRING,
    allowNull: true,
  },
});

module.exports = { sequelize, Todo };
```

- **`Sequelize`**: Sequelize is an ORM for Node.js that provides a way to interact with the database in a more structured and convenient manner.
- **`Todo`**: Defines the `Todo` model with `id`, `name`, and `description` fields. The `id` is set as the primary key and auto-increments with each new entry.
- **`sequelize.sync()`**: Synchronizes the model with the SQLite database, ensuring the table structure is created in the database.

---

### 3. **routes/todos.js**

```javascript
const express = require('express');
const { Todo } = require('../models/todo');

const router = express.Router();

// Create a new To-Do item
router.post('/', async (req, res) => {
  try {
    const { name, description } = req.body;
    if (!name) {
      return res.status(400).json({ error: 'Name is required' });
    }
    const todo = await Todo.create({ name, description });
    res.status(201).json(todo);
  } catch (error) {
    res.status(500).json({ error: 'Failed to create To-Do item' });
  }
});
```

- **`express.Router()`**: This is used to define the routes for the `/todos` endpoint.
- **Route Handlers**:
  - **`POST /todos`**: Creates a new To-Do item. It checks if the `name` field is provided in the request body and then creates the item using Sequelize.
  - **`GET /todos`**: Retrieves all To-Do items from the database.
  - **`GET /todos/:id`**: Retrieves a specific To-Do item by its ID.
  - **`PUT /todos/:id`**: Updates a To-Do item’s `name` and `description`.
  - **`DELETE /todos/:id`**: Deletes a To-Do item by its ID.
