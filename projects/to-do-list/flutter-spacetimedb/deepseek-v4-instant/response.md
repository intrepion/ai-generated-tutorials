<!--
Source: Flutter SpacetimeDB Todo Tutorial - DeepSeek
URL: https://chat.deepseek.com/a/chat/s/0b02c251-e27c-4957-8d3b-53525698c173
Extracted: 2026-06-19
-->

# Building a Real-Time To-Do List with Flutter and SpacetimeDB

If you've ever built a Flutter app that needs a backend, you know the drill: set up a server, write REST APIs, handle WebSocket connections for real-time updates, manage authentication, and then figure out how to keep everything in sync. It's a lot of moving parts.

SpacetimeDB changes that equation entirely. It's a database that *is* your backend — you write your application logic directly inside the database using Rust reducers, and clients connect directly to it. Every client gets a live, synced view of the data automatically. No REST layer, no GraphQL schema, no polling.1820

In this tutorial, we'll build a complete To-Do List app with Flutter and SpacetimeDB. We'll cover the full CRUD cycle: creating, reading, updating, toggling completion status, and deleting todo items. Let's dive in!

## What We're Building

A simple but fully functional todo app where users can:

- View the entire list of todos
- Create new todo items
- Read/View existing todo items
- Update todo descriptions
- Toggle completion status
- Delete todos

All updates will sync in real-time across all connected clients — because that's just how SpacetimeDB works out of the box.

## Prerequisites

Before we start, make sure you have:

- **Flutter SDK** installed (latest stable version)
- **Rust and Cargo** installed — SpacetimeDB uses Rust for its server-side logic38
- **SpacetimeDB CLI** — install it with:
  bashCopyDownload
  ```
  cargo install spacetimedb-cli
  ```
  
  38

## Step 1: Create the SpacetimeDB Module

SpacetimeDB modules are written in Rust. They define your database tables (the schema) and reducers (the logic that modifies data).38

### Initialize the Project

Create a new SpacetimeDB project:
bashCopyDownload
```
spacetime init todo-backend
cd todo-backend
```

### Define the Todo Table

Open `src/lib.rs` and define your table schema. A `Todo` will have an ID, a description, a completion status, and a reference to who created it:
rustCopyDownload
```
use spacetimedb::{spacetimedb, Identity, ReducerContext};

#[spacetimedb(table)]
pub struct Todo {
    #[primary_key]
    pub id: u32,
    pub description: String,
    pub completed: bool,
    pub created_by: Identity,
}
```

The `#[spacetimedb(table)]` attribute tells SpacetimeDB to create a database table from this struct.38 The `#[primary_key]` attribute on `id` makes it the unique identifier for each todo.38

### Implement Reducers

Now we'll write the reducer functions that clients will call to modify data. Reducers are functions marked with `#[spacetimedb(reducer)]` that clients can invoke to change the database state.38

#### Create Todo
rustCopyDownload
```
#[spacetimedb(reducer)]
pub fn create_todo(ctx: ReducerContext, id: u32, description: String) -> Result<(), String> {
    let creator_identity = ctx.identity();
    
    if Todo::filter_by_id(&id).count() > 0 {
        return Err(format!("Todo with ID {} already exists.", id));
    }
    
    Todo::insert(Todo {
        id,
        description,
        completed: false,
        created_by: creator_identity,
    });
    
    Ok(())
}
```

38

#### Update Todo
rustCopyDownload
```
#[spacetimedb(reducer)]
pub fn update_todo(ctx: ReducerContext, id: u32, new_description: String) -> Result<(), String> {
    let todo = Todo::filter_by_id(&id).first();
    
    match todo {
        Some(mut t) => {
            t.description = new_description;
            Todo::update(t);
            Ok(())
        }
        None => Err(format!("Todo with ID {} not found.", id)),
    }
}
```

#### Toggle Completion Status
rustCopyDownload
```
#[spacetimedb(reducer)]
pub fn toggle_todo(ctx: ReducerContext, id: u32) -> Result<(), String> {
    let todo = Todo::filter_by_id(&id).first();
    
    match todo {
        Some(mut t) => {
            t.completed = !t.completed;
            Todo::update(t);
            Ok(())
        }
        None => Err(format!("Todo with ID {} not found.", id)),
    }
}
```

#### Delete Todo
rustCopyDownload
```
#[spacetimedb(reducer)]
pub fn delete_todo(ctx: ReducerContext, id: u32) -> Result<(), String> {
    let todo = Todo::filter_by_id(&id).first();
    
    match todo {
        Some(t) => {
            Todo::delete(t);
            Ok(())
        }
        None => Err(format!("Todo with ID {} not found.", id)),
    }
}
```

Reducers are the only way to modify data in SpacetimeDB. This centralizes your business logic, ensures validation happens on the server, and makes your application more secure and deterministic.38

### Start the Local Server

Run your SpacetimeDB instance locally:
bashCopyDownload
```
spacetime start
```

Then publish your module:
bashCopyDownload
```
spacetime publish todo-app
```

## Step 2: Set Up the Flutter Project

### Create a New Flutter Project
bashCopyDownload
```
flutter create todo_flutter
cd todo_flutter
```

### Add the SpacetimeDB SDK

Add the `spacetimedb_sdk` package to your `pubspec.yaml`:
yamlCopyDownload
```
dependencies:
  flutter:
    sdk: flutter
  spacetimedb_sdk: ^2.1.0
```

Run `flutter pub get`.

### Generate Client Bindings

The SpacetimeDB CLI can generate type-safe Dart client code from your module:
bashCopyDownload
```
spacetime generate --lang dart --out-dir lib/generated todo-app
```

1

This creates a `client.dart` file with typed classes for your tables, reducers, and data types. The generated code makes every table a `ValueNotifier` and every reducer a typed async method.18

## Step 3: Connect to SpacetimeDB

Create a service class to manage the database connection. In `lib/services/db_service.dart`:
dartCopyDownload
```
import 'package:spacetimedb_sdk/spacetimedb_sdk.dart';
import '../generated/client.dart';

class DbService {
  static final DbService _instance = DbService._internal();
  factory DbService() => _instance;
  DbService._internal();

  SpacetimeDbClient? _client;
  
  SpacetimeDbClient get client {
    if (_client == null) {
      throw Exception('Database not connected. Call connect() first.');
    }
    return _client!;
  }

  bool get isConnected => _client != null;

  Future<void> connect() async {
    try {
      _client = await SpacetimeDbClient.connect(
        host: 'localhost:3000',  // Replace with your server address
        database: 'todo-app',
        ssl: false,
        authStorage: InMemoryTokenStore(),
      );
      
      // Subscribe to all todos
      await _client!.subscriptions.subscribe([
        'SELECT * FROM todo',
      ]);
      [reference:14]
      
      print('Connected to SpacetimeDB');
    } catch (e) {
      print('Connection failed: $e');
      rethrow;
    }
  }

  void disconnect() {
    _client?.connection.disconnect();
    _client = null;
  }
}
```

The subscription tells SpacetimeDB which data to sync to this client. Once subscribed, the SDK maintains a local cache that updates automatically whenever the data changes on the server.53

## Step 4: Build the UI with CRUD Operations

Now let's build the Flutter UI. We'll use a `ValueListenableBuilder` to react to changes in the todo table — whenever any client adds, updates, or deletes a todo, the UI will rebuild automatically.18

### The Todo Model

Create a simple wrapper for the generated `Todo` class in `lib/models/todo_model.dart`:
dartCopyDownload
```
import '../generated/client.dart';

class TodoModel {
  final Todo todo;
  
  TodoModel(this.todo);
  
  int get id => todo.id;
  String get description => todo.description;
  bool get completed => todo.completed;
  
  TodoModel copyWith({String? description, bool? completed}) {
    return TodoModel(Todo(
      id: todo.id,
      description: description ?? todo.description,
      completed: completed ?? todo.completed,
      createdBy: todo.createdBy,
    ));
  }
}
```

### The Main UI

Here's the main screen that displays the todo list and handles all CRUD operations. In `lib/main.dart`:
dartCopyDownload
```
import 'package:flutter/material.dart';
import 'package:spacetimedb_sdk/spacetimedb_sdk.dart';
import 'services/db_service.dart';
import 'generated/client.dart';

void main() => runApp(TodoApp());

class TodoApp extends StatelessWidget {
  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      title: 'Flutter Todo + SpacetimeDB',
      theme: ThemeData(primarySwatch: Colors.blue),
      home: TodoListScreen(),
    );
  }
}

class TodoListScreen extends StatefulWidget {
  @override
  _TodoListScreenState createState() => _TodoListScreenState();
}

class _TodoListScreenState extends State<TodoListScreen> {
  final DbService _db = DbService();
  bool _isLoading = true;
  String? _error;

  // Text controllers for creating/editing
  final TextEditingController _newTodoController = TextEditingController();
  final TextEditingController _editController = TextEditingController();

  @override
  void initState() {
    super.initState();
    _connect();
  }

  Future<void> _connect() async {
    try {
      await _db.connect();
      if (mounted) {
        setState(() {
          _isLoading = false;
        });
      }
    } catch (e) {
      if (mounted) {
        setState(() {
          _isLoading = false;
          _error = 'Failed to connect: $e';
        });
      }
    }
  }

  @override
  void dispose() {
    _newTodoController.dispose();
    _editController.dispose();
    _db.disconnect();
    super.dispose();
  }

  // --- CREATE ---
  Future<void> _createTodo() async {
    final description = _newTodoController.text.trim();
    if (description.isEmpty) return;

    // Generate a simple ID (in production, use a better strategy)
    final todos = _db.client.todo.iter().toList();
    final nextId = todos.isEmpty ? 1 : todos.map((t) => t.id).reduce((a, b) => a > b ? a : b) + 1;

    try {
      await _db.client.reducers.createTodo(
        id: nextId,
        description: description,
      );
      _newTodoController.clear();
    } on SpacetimeDbException catch (e) {
      _showError('Failed to create todo: $e');
    }
  }

  // --- READ (displayed via ValueListenableBuilder) ---
  // --- UPDATE ---
  Future<void> _updateTodo(int id, String newDescription) async {
    if (newDescription.trim().isEmpty) return;

    try {
      await _db.client.reducers.updateTodo(
        id: id,
        newDescription: newDescription.trim(),
      );
    } on SpacetimeDbException catch (e) {
      _showError('Failed to update todo: $e');
    }
  }

  // --- TOGGLE COMPLETION ---
  Future<void> _toggleTodo(int id) async {
    try {
      await _db.client.reducers.toggleTodo(id: id);
    } on SpacetimeDbException catch (e) {
      _showError('Failed to toggle todo: $e');
    }
  }

  // --- DELETE ---
  Future<void> _deleteTodo(int id) async {
    try {
      await _db.client.reducers.deleteTodo(id: id);
    } on SpacetimeDbException catch (e) {
      _showError('Failed to delete todo: $e');
    }
  }

  void _showError(String message) {
    ScaffoldMessenger.of(context).showSnackBar(
      SnackBar(content: Text(message), backgroundColor: Colors.red),
    );
  }

  void _showEditDialog(Todo todo) {
    _editController.text = todo.description;
    showDialog(
      context: context,
      builder: (ctx) => AlertDialog(
        title: Text('Edit Todo'),
        content: TextField(
          controller: _editController,
          autofocus: true,
          decoration: InputDecoration(hintText: 'Update description'),
        ),
        actions: [
          TextButton(
            onPressed: () => Navigator.pop(ctx),
            child: Text('Cancel'),
          ),
          TextButton(
            onPressed: () {
              _updateTodo(todo.id, _editController.text);
              Navigator.pop(ctx);
            },
            child: Text('Save'),
          ),
        ],
      ),
    );
  }

  @override
  Widget build(BuildContext context) {
    if (_isLoading) {
      return Scaffold(
        appBar: AppBar(title: Text('Todo List')),
        body: Center(child: CircularProgressIndicator()),
      );
    }

    if (_error != null) {
      return Scaffold(
        appBar: AppBar(title: Text('Todo List')),
        body: Center(
          child: Column(
            mainAxisAlignment: MainAxisAlignment.center,
            children: [
              Text(_error!, style: TextStyle(color: Colors.red)),
              ElevatedButton(
                onPressed: () {
                  setState(() {
                    _isLoading = true;
                    _error = null;
                  });
                  _connect();
                },
                child: Text('Retry'),
              ),
            ],
          ),
        ),
      );
    }

    return Scaffold(
      appBar: AppBar(
        title: Text('Todo List'),
        actions: [
          IconButton(
            icon: Icon(Icons.refresh),
            onPressed: () => setState({}),
          ),
        ],
      ),
      body: Column(
        children: [
          // Create todo input
          Padding(
            padding: EdgeInsets.all(16.0),
            child: Row(
              children: [
                Expanded(
                  child: TextField(
                    controller: _newTodoController,
                    decoration: InputDecoration(
                      hintText: 'Enter a new todo...',
                      border: OutlineInputBorder(),
                    ),
                    onSubmitted: (_) => _createTodo(),
                  ),
                ),
                SizedBox(width: 8),
                ElevatedButton(
                  onPressed: _createTodo,
                  child: Text('Add'),
                ),
              ],
            ),
          ),
          
          // Todo list with real-time updates
          Expanded(
            child: ValueListenableBuilder(
              valueListenable: _db.client.todo,
              builder: (context, todoList, child) {
                final todos = todoList.iter().toList();
                
                if (todos.isEmpty) {
                  return Center(
                    child: Text(
                      'No todos yet. Add one above!',
                      style: TextStyle(color: Colors.grey),
                    ),
                  );
                }
                
                return ListView.builder(
                  itemCount: todos.length,
                  itemBuilder: (ctx, index) {
                    final todo = todos[index];
                    return TodoItem(
                      todo: todo,
                      onToggle: () => _toggleTodo(todo.id),
                      onEdit: () => _showEditDialog(todo),
                      onDelete: () => _deleteTodo(todo.id),
                    );
                  },
                );
              },
            ),
          ),
        ],
      ),
    );
  }
}

// --- Todo Item Widget ---
class TodoItem extends StatelessWidget {
  final Todo todo;
  final VoidCallback onToggle;
  final VoidCallback onEdit;
  final VoidCallback onDelete;

  const TodoItem({
    Key? key,
    required this.todo,
    required this.onToggle,
    required this.onEdit,
    required this.onDelete,
  }) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return ListTile(
      leading: Checkbox(
        value: todo.completed,
        onChanged: (_) => onToggle(),
      ),
      title: Text(
        todo.description,
        style: TextStyle(
          decoration: todo.completed ? TextDecoration.lineThrough : null,
          color: todo.completed ? Colors.grey : null,
        ),
      ),
      subtitle: Text('ID: ${todo.id}'),
      trailing: Row(
        mainAxisSize: MainAxisSize.min,
        children: [
          IconButton(
            icon: Icon(Icons.edit, color: Colors.blue),
            onPressed: onEdit,
          ),
          IconButton(
            icon: Icon(Icons.delete, color: Colors.red),
            onPressed: onDelete,
          ),
        ],
      ),
      onTap: onEdit,
    );
  }
}
```

## Understanding the Magic: How It Works

### Real-Time Sync

The `ValueListenableBuilder` listening to `_db.client.todo` is the key to real-time updates. Whenever any client (including this one) inserts, updates, or deletes a todo, the SDK updates the local cache and notifies all listeners.18 The UI rebuilds automatically — no polling, no manual refresh, no WebSocket management on your part.

### Optimistic Updates

The SDK applies writes optimistically: when you call a reducer, the change appears in the UI instantly. If the server confirms it, nothing happens. If the server rejects it (e.g., due to a validation error), the SDK rolls back automatically.18 This makes the app feel snappy and responsive.

### Reducers Are Typed

Because we generated client bindings from our Rust module, the reducer calls are fully typed. Your IDE will autocomplete `createTodo`, `updateTodo`, `toggleTodo`, and `deleteTodo` with the correct parameter types.18 No more stringly-typed API endpoints!

## Running the App

- Make sure your SpacetimeDB server is running:
  bashCopyDownload
  ```
  spacetime start
  ```
- Publish your module (if you haven't already):
  bashCopyDownload
  ```
  spacetime publish todo-app
  ```
- Run the Flutter app:
  bashCopyDownload
  ```
  flutter run
  ```

Open the app on multiple devices or simulators — add a todo on one, and watch it appear on the others instantly. That's the power of SpacetimeDB.

## Next Steps

This tutorial covered the basics, but there's so much more you can do:

- **Authentication**: SpacetimeDB supports token-based authentication. You can integrate with Clerk or your own auth system.1
- **Offline support**: Flip on `JsonFileStorage` and the SDK will cache data locally and queue writes when offline.18
- **Row-level watching**: Use `rowNotifier(id)` to watch individual rows instead of the whole table for better performance.18
- **Views**: SpacetimeDB v2 supports server-defined views for identity-scoped data projections.1

The complete code for this tutorial is available — build on it, add user authentication, or turn it into a collaborative team task manager. With SpacetimeDB, the backend is already built and real-time by default.

Happy coding! 🚀
