# Actix-web + Diesel + SQLite (Todo Application)

This is a simple **To-Do API** built with **Actix-web** and **SQLite** using **Diesel ORM** for data modeling. The API allows you to perform CRUD (Create, Read, Update, Delete) operations on To-Do items, each with a `name` and an optional `description`.

## Project Structure

```
.
├── migrations                              # Database migrations
│   └── 2024-11-19-072332_create_todos      # Migration for creating the `todos` table
│       ├── down.sql                        # SQL script for rolling back the migration
│       └── up.sql                          # SQL script for applying the migration
├── src                                     # Source code
│   ├── db                                  # Database module
│   │   ├── models.rs                       # Database models
│   │   ├── schema.rs                       # Database schema
│   ├── db.rs                               # Database connection
│   └── main.rs                             # Main application
└── test.db                                 # SQLite database file
```

### 1. **main.rs**: Main server file

This is the entry point of the application. It initializes the server and handles routing.

### 2. **src/db/schema.rs**: Diesel model definition

This file defines the schema for the `Todo` model using Diesel ORM. The model has an `id`, `name`, and `description` field.

---

## Installation

### Running the App

1. Clone the repository or create the project manually.
2. Navigate to the project directory and install dependencies:

```bash
cargo install
```

3. Start the server:

```bash
cargo run --release
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
  "id": "37b72937-8d83-4e83-8e05-cc225d66c9e9",
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
    "id": "37b72937-8d83-4e83-8e05-cc225d66c9e9",
    "name": "Buy groceries",
    "description": "Milk, eggs, bread"
  },
  {
    "id": "8fad0065-42d2-4601-89fe-10ca7098b098",
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
  "id": "37b72937-8d83-4e83-8e05-cc225d66c9e9",
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
  "id": "37b72937-8d83-4e83-8e05-cc225d66c9e9",
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

### 1. **main.rs** (Actix-web server part)

```rust
use actix_web::{web, App, HttpResponse, HttpServer, Responder};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/todos", web::get().to(get_todos))
            .route("/todos/{id}", web::get().to(get_todo_by_id))
            .route("/todos", web::post().to(create_todo))
            .route("/todos/{id}", web::put().to(update_todo))
            .route("/todos/{id}", web::delete().to(delete_todo))
    })
    .bind(("0.0.0.0", 8000))?
    .run()
    .await
}
```

- **`actix_web`**: Actix_web is used to handle HTTP requests.
- **`HttpServer::new()`**: This create the new instance of the HTTP server.
- **`App::new()`**: This creates a new instance of the application.
- **`.route("/todos", web::get().to(get_todos))`**: This defines a route for the `GET /todos` endpoint that calls the `get_todos` function.

---

### 2. **src/main.rs** (Routes part)

```rust
async fn get_todos() -> impl Responder {
    let todos = _db_get_todos();
    HttpResponse::Ok().json(todos)
}

async fn get_todo_by_id(path: web::Path<String>) -> impl Responder {
    let todo = _db_get_todo_by_id(path.into_inner());
    HttpResponse::Ok().json(todo)
}
```

- **`async`**: This keyword is used to define an asynchronous function since the database operations are asynchronous.
- **`fn get_todo_by_id`**: This function is a handler for the _db_get_todo_by_id function from the db module.
- **`impl Responder`**: This is the return type of the function, which indicates that the function will return a response.
- **`HttpResponse::Ok().json(todos)`**: This creates an HTTP response with a status code of `200 OK` and returns the To-Do items as JSON.

---

### 3. **src/db.rs**

```rust

pub fn _db_get_todos() -> Vec<Todo> {
    let mut connection = establish_connection();
    todos
        .limit(5)
        .load::<Todo>(&mut connection)
        .expect("Error loading todos")
}

pub fn _db_get_todo_by_id(key: String) -> Vec<Todo> {
    let mut connection = establish_connection();
    todos
        .find(key)
        .load::<Todo>(&mut connection)
        .expect("Error loading todo")
}
```

- **`establish_connection()`**: This function establishes a connection to the SQLite database using the `DATABASE_URL` environment variable.
- **`todos.limit(5).load::<Todo>(&mut connection)`**: This query loads the first 5 To-Do items from the database.
- **`todos.find(key).load::<Todo>(&mut connection)`**: This query loads a specific To-Do item by its ID from the database.
- **`.expect("Error loading todos")`**: This is used to handle any errors that occur during the database query.
- **`Vec<Todo>`**: This is the return type of the function, which is a vector of `Todo` items.
- **`Todo`**: This is the model defined in the `schema.rs` file using Diesel ORM.

### 4. **src/db/schema.rs**

```rust
diesel::table! {
    todos (id) {
        id -> Text,
        name -> Text,
        description -> Text,
    }
}
```

- **`diesel::table!`**: This macro is used to define a table in the database.
- **`todos (id)`**: This defines the `todos` table with the `id` field as the primary key.
- **`id -> Text`**: This defines the `id` field as a `Text` type.
- **`name -> Text`**: This defines the `name` field as a `Text` type.