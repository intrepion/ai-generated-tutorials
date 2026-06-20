<!--
Source: Flutter To-Do List Tutorial
URL: https://chatgpt.com/c/6a34c170-0708-83e8-b338-c82c16900714
Extracted: 2026-06-19
-->

# Building a Real-Time To-Do List with Flutter and SpacetimeDB

Most CRUD tutorials stop at a database and a REST API. With SpacetimeDB, you can skip the traditional backend entirely and build a real-time application where database changes automatically synchronize with connected clients.

In this tutorial, we'll build a simple To-Do List application using Flutter and SpacetimeDB that supports:

- View all to-do items
- Create new items
- Read item details
- Update existing items
- Toggle completion status
- Delete items
- Real-time synchronization across devices

By the end, you'll have a complete CRUD application powered by SpacetimeDB.

# What is SpacetimeDB?

SpacetimeDB is a multiplayer database that combines:

- Database
- Backend server
- Real-time synchronization
- Business logic execution

Instead of writing:

- Flutter UI
- REST API
- Database layer
- WebSocket synchronization

You write:

- Flutter UI
- SpacetimeDB schema
- SpacetimeDB reducers

The database automatically synchronizes state changes to connected clients.

# Project Structure

```
todo_flutter/
├── lib/
│   ├── main.dart
│   ├── models/
│   │   └── todo.dart
│   ├── services/
│   │   └── spacetime_service.dart
│   └── screens/
│       └── todo_screen.dart
│
└── spacetime/
    └── src/
        └── lib.rs
```

# Step 1: Create the SpacetimeDB Module

Initialize a new SpacetimeDB project:

```
spacetime init todo-backend
cd todo-backend
```

Open:

```
src/lib.rs
```

# Step 2: Define the To-Do Table

Create a table that stores our tasks.

```
use spacetimedb::{table, reducer, Identity};

#[table(name = todo)]
pub struct Todo {
    #[primary_key]
    pub id: u64,

    pub title: String,

    pub description: String,

    pub completed: bool,

    pub owner: Identity,
}
```

Each row contains:
FieldPurposeidUnique identifiertitleTask titledescriptionTask detailscompletedCompletion statusownerUser identity
# Step 3: Create a To-Do Item

Reducers are similar to API endpoints.

```
#[reducer]
pub fn create_todo(
    ctx: &ReducerContext,
    title: String,
    description: String,
) {
    let id = ctx.db.todo().count() as u64 + 1;

    ctx.db.todo().insert(Todo {
        id,
        title,
        description,
        completed: false,
        owner: ctx.sender,
    });
}
```

This reducer:

- Generates an ID
- Creates a task
- Stores it in the database
- Synchronizes it to all clients

# Step 4: Read a To-Do Item

We don't need a reducer for reads.

Clients subscribe directly to tables.

Example query:

```
SELECT * FROM todo
```

SpacetimeDB automatically streams updates whenever rows change.

# Step 5: Update a To-Do Item

```
#[reducer]
pub fn update_todo(
    ctx: &ReducerContext,
    id: u64,
    title: String,
    description: String,
) {
    if let Some(mut todo) = ctx.db.todo().id().find(id) {

        todo.title = title;
        todo.description = description;

        ctx.db.todo().id().update(todo);
    }
}
```

This modifies an existing task.

# Step 6: Toggle Completion Status

A dedicated reducer makes completion easy.

```
#[reducer]
pub fn toggle_complete(
    ctx: &ReducerContext,
    id: u64,
) {
    if let Some(mut todo) = ctx.db.todo().id().find(id) {

        todo.completed = !todo.completed;

        ctx.db.todo().id().update(todo);
    }
}
```

Every connected client instantly sees the update.

# Step 7: Delete a To-Do Item

```
#[reducer]
pub fn delete_todo(
    ctx: &ReducerContext,
    id: u64,
) {
    ctx.db.todo().id().delete(id);
}
```

Simple and effective.

# Step 8: Publish the Module

Build:

```
spacetime build
```

Start locally:

```
spacetime start
```

Publish:

```
spacetime publish todo
```

Your database backend is now running.

# Step 9: Create the Flutter Project

```
flutter create todo_flutter
cd todo_flutter
```

Add dependencies:

```
dependencies:
  flutter:
    sdk: flutter

  provider: ^6.1.2
```

# Step 10: Create the To-Do Model

```
class Todo {
  final int id;
  final String title;
  final String description;
  final bool completed;

  Todo({
    required this.id,
    required this.title,
    required this.description,
    required this.completed,
  });
}
```

# Step 11: Create a Spacetime Service

This service maintains the synchronized task list.

```
import 'package:flutter/foundation.dart';

class SpacetimeService extends ChangeNotifier {
  final List<Todo> _todos = [];

  List<Todo> get todos => _todos;

  Future<void> connect() async {
    // Connect to SpacetimeDB

    // Subscribe to:
    // SELECT * FROM todo

    // When updates arrive:
    // refresh _todos
    // notifyListeners();
  }

  Future<void> createTodo(
    String title,
    String description,
  ) async {
    // invoke reducer create_todo
  }

  Future<void> updateTodo(
    Todo todo,
  ) async {
    // invoke reducer update_todo
  }

  Future<void> toggleComplete(
    int id,
  ) async {
    // invoke reducer toggle_complete
  }

  Future<void> deleteTodo(
    int id,
  ) async {
    // invoke reducer delete_todo
  }
}
```

The exact generated API will depend on the version of the SpacetimeDB Flutter SDK you use.

# Step 12: Display the List

```
Consumer<SpacetimeService>(
  builder: (context, service, _) {

    return ListView.builder(
      itemCount: service.todos.length,

      itemBuilder: (context, index) {

        final todo = service.todos[index];

        return ListTile(
          title: Text(todo.title),

          subtitle: Text(todo.description),

          trailing: Checkbox(
            value: todo.completed,

            onChanged: (_) {
              service.toggleComplete(todo.id);
            },
          ),
        );
      },
    );
  },
);
```

This automatically refreshes whenever SpacetimeDB sends updates.

# Step 13: Create New Items

Floating action button:

```
FloatingActionButton(
  onPressed: () {

    service.createTodo(
      "Buy Milk",
      "Remember to buy milk",
    );
  },

  child: Icon(Icons.add),
)
```

# Step 14: Edit Existing Items

Open an edit dialog.

```
await showDialog(
  context: context,

  builder: (_) {

    return AlertDialog(
      title: const Text("Edit Task"),

      content: TextField(
        controller: controller,
      ),

      actions: [
        ElevatedButton(
          onPressed: () {

            service.updateTodo(
              updatedTodo,
            );
          },

          child: const Text("Save"),
        )
      ],
    );
  },
);
```

# Step 15: Delete Items

Add a delete button.

```
IconButton(
  icon: const Icon(Icons.delete),

  onPressed: () {
    service.deleteTodo(todo.id);
  },
)
```

# Testing Real-Time Synchronization

Open two app instances.

Instance A:

```
Create Task
```

Instance B immediately receives:

```
Create Task
```

Toggle completion on Instance B:

```
✓ Create Task
```

Instance A updates automatically.

No REST polling.

No WebSocket implementation.

No backend synchronization code.

SpacetimeDB handles it.

# Final Result

Our application now supports:

✅ Create To-Do Item

✅ Read To-Do Item

✅ Update To-Do Item

✅ Toggle Completion

✅ Delete To-Do Item

✅ Real-Time Synchronization

# Where to Go Next

Once the basic CRUD application works, consider adding:

### Authentication

```
owner: Identity
```

Only allow users to modify their own tasks.

### Due Dates

```
due_date: Timestamp
```

Sort upcoming work automatically.

### Categories

```
Work
Personal
Shopping
Fitness
```

### Offline Support

Store local changes and synchronize when connectivity returns.

### Shared Lists

Allow multiple users to collaborate on a single task list.

### Notifications

Send reminders for upcoming tasks.

# Conclusion

Flutter provides a productive cross-platform UI framework, while SpacetimeDB removes much of the traditional backend complexity. Together they make it possible to build real-time applications with surprisingly little code.

For a simple To-Do application, the amount of infrastructure eliminated is substantial:

- No REST API
- No ORM
- No WebSocket server
- No synchronization layer
- No polling

Just reducers, tables, subscriptions, and a Flutter UI.

That makes this project an excellent introduction to both Flutter and SpacetimeDB before moving on to multiplayer apps, collaborative tools, dashboards, games, or real-time business applications.

One thing I'd add if you plan to publish this: include the exact Flutter client SDK setup and generated client code for the specific SpacetimeDB version you're targeting. Their APIs have evolved rapidly, and readers will get the most value if the code compiles against a known version rather than using pseudocode service methods.

ChatGPT can make mistakes. Check important info.
