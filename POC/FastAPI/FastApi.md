### **README.md**

# FastAPI + SQLModel + SQLite (Todo Application)

This is a simple **FastAPI** project using **SQLModel** and **SQLite** for managing a "Todo" application. You can add, list, and retrieve todos via RESTful APIs.

---

## **Project Structure**

- **`models.py`**: Defines the database model for the `Todo` table.
- **`crud.py`**: Contains helper functions to interact with the database (CRUD operations).
- **`database.py`**: Manages database connection and initialization.
- **`main.py`**: The entry point for the FastAPI application, defining the API endpoints.

---

## **Code Explanation**

### **Model: Todo**

The `Todo` model is a SQLModel class that represents a table in the SQLite database.

```python
class Todo(SQLModel, table=True):
    id: Optional[int] = Field(default=None, primary_key=True)  # Auto-incrementing primary key
    name: str                                                # Name of the todo task
    description: Optional[str] = None                        # Optional description
```

- `id`: Unique identifier for each task.
- `name`: Task name (required).
- `description`: Optional additional details about the task.

---

### **API Endpoints**

#### **1. Create a Todo**

- **Endpoint**: `POST /todos/`
- **Description**: Add a new task to the database.

#### **2. List All Todos**

- **Endpoint**: `GET /todos/`
- **Description**: Retrieve all tasks.

#### **3. Get a Specific Todo**

- **Endpoint**: `GET /todos/{id}`
- **Description**: Retrieve a specific task by its ID.

---

## **Usage with cURL**

### **Base URL**

- Local development server: `http://127.0.0.1:8000`

---

### **1. Create a Todo**

Use the `POST` method to create a new task.

#### **Command**

```bash
curl -X POST "http://127.0.0.1:8000/todos/" \
-H "Content-Type: application/json" \
-d '{"name": "Buy groceries", "description": "Milk, Bread, Eggs"}'
```

#### **Response**

```json
{
  "id": 1,
  "name": "Buy groceries",
  "description": "Milk, Bread, Eggs"
}
```

---

### **2. List All Todos**

Use the `GET` method to retrieve all tasks.

#### **Command**

```bash
curl -X GET "http://127.0.0.1:8000/todos/"
```

#### **Response**

```json
[
  {
    "id": 1,
    "name": "Buy groceries",
    "description": "Milk, Bread, Eggs"
  }
]
```

---

### **3. Get a Specific Todo**

Use the `GET` method to retrieve a task by its ID.

#### **Command**

```bash
curl -X GET "http://127.0.0.1:8000/todos/1"
```

#### **Response**

```json
{
  "id": 1,
  "name": "Buy groceries",
  "description": "Milk, Bread, Eggs"
}
```

---

### **4. Update a Specific Todo**

Use the `PUT` method to update a task by its ID.

#### **Command**

```bash
curl -X PUT "http://127.0.0.1:8000/todos/1"
-H "Content-Type: application/json" \
-d '{"name": "Buy groceries", "description": "Milk, Bread, Eggs, Butter"}'
```

#### **Response**

```json
{
  "id": 1,
  "name": "Buy groceries",
  "description": "Milk, Bread, Eggs, Butter"
}
```

### **5. Delete a Specific Todo**

Use the `DELETE` method to delete a task by its ID.

#### **Command**

```bash
curl -X PUT "http://127.0.0.1:8000/todos/1"
```

#### **Response**

```json
{
   "id": 1,
   "name": "Buy groceries",
   "description": "Milk, Bread, Eggs, Butter"
}
```

---

## **Running the Application**

1. **Set up the environment:**

   ```bash
   python3 -m venv env
   source env/bin/activate
   ```

2. **Install dependencies:**

   ```bash
   pip install -r requirements.txt
   ```

3. **Start the server:**

   ```bash
   python3 .
   ```

4. **Visit API documentation:**
   - Swagger UI: [http://127.0.0.1:8000/docs](http://127.0.0.1:8000/docs)
   - ReDoc: [http://127.0.0.1:8000/redoc](http://127.0.0.1:8000/redoc)

---

## **How the Code Works**

1. **`models.py`**
   - Defines the `Todo` class as a database model.
   - SQLModel handles database table creation and schema.

2. **`database.py`**
   - Sets up the SQLite database connection using `SQLModel.create_all`.
   - Provides a `get_session` function for dependency injection in FastAPI routes.

3. **`crud.py`**
   - Functions to create, retrieve all, and retrieve a specific todo task.

4. **`main.py`**
   - Defines FastAPI routes and integrates with the database using dependencies.
   - Automatically initializes the database on application startup.
