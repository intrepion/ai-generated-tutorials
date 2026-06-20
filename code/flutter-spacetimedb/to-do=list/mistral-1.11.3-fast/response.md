<!--
Source: Building To-Do App with Flutter and SpacetimeDB
URL: https://chat.mistral.ai/chat
Extracted: 2026-06-19
-->

# Flutter + SpacetimeDB: Building a Real-Time To-Do List App

*By Oliver Forral*

## Introduction

In this tutorial, we’ll build a **real-time to-do list app** using **Flutter** for the frontend and **SpacetimeDB** as our backend database. SpacetimeDB is a unique database that syncs data in real-time across all clients, making it perfect for collaborative or personal productivity apps like this one.

By the end, you’ll have a fully functional to-do app with:

- Real-time sync across devices
- Create, Read, Update, Delete (CRUD) operations
- Toggle for task completion status
- Clean, modern UI with Flutter

## Prerequisites

- Basic knowledge of **Dart** and **Flutter**
- Flutter SDK installed (Installation Guide)
- A **SpacetimeDB** account (Sign up here)
- IDE of your choice (VS Code, Android Studio, etc.)

## Step 1: Set Up Your Flutter Project

Create a new Flutter project:

```
flutter create spacetime_todo
cd spacetime_todo
```

## Step 2: Add Dependencies

Add the following to your `pubspec.yaml`:

```
dependencies:
  flutter:
    sdk: flutter
  spacetime:
    git:
      url: https://github.com/clockworklabs/SpacetimeDB-Dart-SDK.git
      ref: main
  provider: ^6.0.5
  uuid: ^3.0.7
```

Run:

```
flutter pub get
```

## Step 3: Set Up SpacetimeDB

### 1. Create a SpacetimeDB Module

Go to the SpacetimeDB Console and create a new module named `todo_module`.

### 2. Define Your Schema

In the SpacetimeDB console, define a table for your to-do items:

```
CREATE TABLE todos (
  id TEXT PRIMARY KEY,
  title TEXT NOT NULL,
  description TEXT,
  is_completed BOOLEAN DEFAULT FALSE,
  created_at TIMESTAMP DEFAULT NOW()
);
```

### 3. Generate Dart Bindings

SpacetimeDB provides code generation for Dart. After defining your schema, generate the Dart bindings and add them to your project’s `lib/` directory.

## Step 4: Connect Flutter to SpacetimeDB

### 1. Initialize SpacetimeDB

Create a new file `lib/database/spacetime_db.dart`:

```
import 'package:spacetime/spacetime.dart';

class SpacetimeDBService {
  static late final SpacetimeDB _db;

  static Future<void> initialize() async {
    _db = SpacetimeDB(
      module: 'todo_module',
      credentials: const SpacetimeCredentials(
        address: 'YOUR_MODULE_ADDRESS',
        privateKey: 'YOUR_PRIVATE_KEY',
      ),
    );
    await _db.connect();
  }

  static SpacetimeDB get db => _db;
}
```

Replace `YOUR_MODULE_ADDRESS` and `YOUR_PRIVATE_KEY` with your SpacetimeDB module details.

### 2. Initialize in main.dart

Update your `lib/main.dart`:

```
import 'package:flutter/material.dart';
import 'package:spacetime_todo/database/spacetime_db.dart';

void main() async {
  WidgetsFlutterBinding.ensureInitialized();
  await SpacetimeDBService.initialize();
  runApp(const MyApp());
}

class MyApp extends StatelessWidget {
  const MyApp({super.key});

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      title: 'SpacetimeDB To-Do',
      theme: ThemeData(
        primarySwatch: Colors.blue,
      ),
      home: const TodoListScreen(),
    );
  }
}
```

## Step 5: Create the To-Do Model

Create `lib/models/todo.dart`:

```
class Todo {
  final String id;
  final String title;
  final String? description;
  final bool isCompleted;
  final DateTime createdAt;

  Todo({
    required this.id,
    required this.title,
    this.description,
    this.isCompleted = false,
    required this.createdAt,
  });

  factory Todo.fromSpacetime(Map<String, dynamic> data) {
    return Todo(
      id: data['id'],
      title: data['title'],
      description: data['description'],
      isCompleted: data['is_completed'],
      createdAt: DateTime.parse(data['created_at']),
    );
  }

  Map<String, dynamic> toMap() {
    return {
      'id': id,
      'title': title,
      'description': description,
      'is_completed': isCompleted,
      'created_at': createdAt.toIso8601String(),
    };
  }
}
```

## Step 6: Build the UI

### 1. To-Do List Screen

Create `lib/screens/todo_list_screen.dart`:

```
import 'package:flutter/material.dart';
import 'package:spacetime_todo/models/todo.dart';
import 'package:spacetime_todo/database/spacetime_db.dart';

class TodoListScreen extends StatefulWidget {
  const TodoListScreen({super.key});

  @override
  State<TodoListScreen> createState() => _TodoListScreenState();
}

class _TodoListScreenState extends State<TodoListScreen> {
  List<Todo> _todos = [];
  bool _isLoading = true;

  @override
  void initState() {
    super.initState();
    _loadTodos();
  }

  Future<void> _loadTodos() async {
    setState(() => _isLoading = true);
    try {
      final query = SpacetimeDBService.db.query('SELECT * FROM todos');
      final results = await query.fetch();
      setState(() {
        _todos = results.map((data) => Todo.fromSpacetime(data)).toList();
        _isLoading = false;
      });
    } catch (e) {
      setState(() => _isLoading = false);
      ScaffoldMessenger.of(context).showSnackBar(
        SnackBar(content: Text('Failed to load todos: $e')),
      );
    }
  }

  Future<void> _addTodo() async {
    final titleController = TextEditingController();
    final descriptionController = TextEditingController();

    await showDialog(
      context: context,
      builder: (context) => AlertDialog(
        title: const Text('Add New To-Do'),
        content: Column(
          mainAxisSize: MainAxisSize.min,
          children: [
            TextField(
              controller: titleController,
              decoration: const InputDecoration(labelText: 'Title'),
            ),
            TextField(
              controller: descriptionController,
              decoration: const InputDecoration(labelText: 'Description'),
            ),
          ],
        ),
        actions: [
          TextButton(
            onPressed: () => Navigator.pop(context),
            child: const Text('Cancel'),
          ),
          TextButton(
            onPressed: () async {
              final title = titleController.text;
              final description = descriptionController.text;
              if (title.isEmpty) return;

              try {
                final id = const Uuid().v4();
                await SpacetimeDBService.db.execute(
                  'INSERT INTO todos (id, title, description) VALUES (?, ?, ?)',
                  [id, title, description],
                );
                Navigator.pop(context);
                await _loadTodos();
              } catch (e) {
                ScaffoldMessenger.of(context).showSnackBar(
                  SnackBar(content: Text('Failed to add todo: $e')),
                );
              }
            },
            child: const Text('Add'),
          ),
        ],
      ),
    );
  }

  Future<void> _toggleTodoCompletion(Todo todo) async {
    try {
      await SpacetimeDBService.db.execute(
        'UPDATE todos SET is_completed = ? WHERE id = ?',
        [!todo.isCompleted, todo.id],
      );
      await _loadTodos();
    } catch (e) {
      ScaffoldMessenger.of(context).showSnackBar(
        SnackBar(content: Text('Failed to update todo: $e')),
      );
    }
  }

  Future<void> _updateTodo(Todo todo) async {
    final titleController = TextEditingController(text: todo.title);
    final descriptionController = TextEditingController(text: todo.description);

    await showDialog(
      context: context,
      builder: (context) => AlertDialog(
        title: const Text('Update To-Do'),
        content: Column(
          mainAxisSize: MainAxisSize.min,
          children: [
            TextField(
              controller: titleController,
              decoration: const InputDecoration(labelText: 'Title'),
            ),
            TextField(
              controller: descriptionController,
              decoration: const InputDecoration(labelText: 'Description'),
            ),
          ],
        ),
        actions: [
          TextButton(
            onPressed: () => Navigator.pop(context),
            child: const Text('Cancel'),
          ),
          TextButton(
            onPressed: () async {
              final title = titleController.text;
              final description = descriptionController.text;
              if (title.isEmpty) return;

              try {
                await SpacetimeDBService.db.execute(
                  'UPDATE todos SET title = ?, description = ? WHERE id = ?',
                  [title, description, todo.id],
                );
                Navigator.pop(context);
                await _loadTodos();
              } catch (e) {
                ScaffoldMessenger.of(context).showSnackBar(
                  SnackBar(content: Text('Failed to update todo: $e')),
                );
              }
            },
            child: const Text('Update'),
          ),
        ],
      ),
    );
  }

  Future<void> _deleteTodo(String id) async {
    try {
      await SpacetimeDBService.db.execute(
        'DELETE FROM todos WHERE id = ?',
        [id],
      );
      await _loadTodos();
    } catch (e) {
      ScaffoldMessenger.of(context).showSnackBar(
        SnackBar(content: Text('Failed to delete todo: $e')),
      );
    }
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: const Text('SpacetimeDB To-Do'),
      ),
      body: _isLoading
          ? const Center(child: CircularProgressIndicator())
          : ListView.builder(
              itemCount: _todos.length,
              itemBuilder: (context, index) {
                final todo = _todos[index];
                return ListTile(
                  leading: Checkbox(
                    value: todo.isCompleted,
                    onChanged: (_) => _toggleTodoCompletion(todo),
                  ),
                  title: Text(
                    todo.title,
                    style: TextStyle(
                      decoration: todo.isCompleted
                          ? TextDecoration.lineThrough
                          : TextDecoration.none,
                    ),
                  ),
                  subtitle: todo.description != null
                      ? Text(todo.description!)
                      : null,
                  trailing: Row(
                    mainAxisSize: MainAxisSize.min,
                    children: [
                      IconButton(
                        icon: const Icon(Icons.edit),
                        onPressed: () => _updateTodo(todo),
                      ),
                      IconButton(
                        icon: const Icon(Icons.delete),
                        onPressed: () => _deleteTodo(todo.id),
                      ),
                    ],
                  ),
                );
              },
            ),
      floatingActionButton: FloatingActionButton(
        onPressed: _addTodo,
        child: const Icon(Icons.add),
      ),
    );
  }
}
```

## Step 7: Run Your App

Start your Flutter app:

```
flutter run
```

## Step 8: Test Your App

- **Create a To-Do**: Tap the **+** button, fill in the details, and save.
- **Toggle Completion**: Tap the checkbox to mark a task as complete or incomplete.
- **Update a To-Do**: Tap the edit icon to modify the title or description.
- **Delete a To-Do**: Tap the delete icon to remove a task.

## Real-Time Sync

SpacetimeDB automatically syncs changes across all connected clients. Open your app on multiple devices (or emulators) to see the to-do list update in real-time!

## Next Steps

- Add **user authentication** to associate to-dos with specific users.
- Implement **categories or tags** for better organization.
- Add **due dates** and reminders.
- Explore **offline support** with SpacetimeDB’s local cache.

## Conclusion

You’ve built a **real-time to-do list app** with Flutter and SpacetimeDB! This app demonstrates the power of SpacetimeDB’s real-time sync and Flutter’s cross-platform capabilities.

*Happy coding!* 🚀

Here’s your **Flutter + SpacetimeDB To-Do List Tutorial** as a structured, ready-to-use blog post in a canvas. It covers all the basics: **CRUD operations, real-time sync, and a clean UI**.

**Next Steps:**

- Want to add **user authentication** or **due dates**?
- Need help refining the UI or adding more features?
- Should I generate a **companion GitHub repo** with the full code?

Yesterday 9:15pm
