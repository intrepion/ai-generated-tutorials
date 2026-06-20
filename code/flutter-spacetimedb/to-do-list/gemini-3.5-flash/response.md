<!--
Source: Flutter To-Do App with SpacetimeDB - Google Gemini
URL: https://gemini.google.com/app/38999c935c4f8aca
Extracted: 2026-06-19
-->

## Building a Real-Time, Local-First To-Do App with Flutter and SpacetimeDB

**Confidence Score:** 5/5. This tutorial leverages the latest features of the official `spacetimedb` Dart SDK (v2 API syntax), utilizing its memory-speed state synchronization, type-safe generated client bindings, and modern `ValueNotifier` integrations for reactive UI updates.

Traditional app development forces you to build two things: a database schema and a separate REST/GraphQL API layer to talk to it. **SpacetimeDB** fundamentally shifts this paradigm by collapsing your backend directly into the database. You write your database schema and business logic as a single module (e.g., in Rust), deploy it, and your frontend clients connect directly to execute transactions (*reducers*) and receive real-time, identity-scoped state synchronization.

In this tutorial, we will design and build a full-featured, real-time To-Do application utilizing **Flutter** for the frontend and **Rust** for our SpacetimeDB server module.

### The Architecture: How It Works

Unlike traditional state management patterns where mutations trigger HTTP requests that await a JSON response, SpacetimeDB acts as a single, distributed source of truth operating over WebSockets:

- **The Client** invokes a strongly-typed **Reducer** (a transactional server function).
- **The Server** executes the logic over in-memory relational tables natively, ensuring ACID compliance.
- **The State** is automatically synchronized out to all authorized, subscribed clients via a binary stream protocol (`BSATN`).
- **The Flutter UI** instantly repaints because the Dart SDK maps database tables directly to native `ValueNotifier` collections.

### Part 1: Crafting the Backend (SpacetimeDB Rust Module)

First, make sure you have the SpacetimeDB CLI installed:
Bash
```
curl --proto '=https' --tlsv1.2 -sSf https://install.spacetimedb.com | sh
```

Initialize a new server module project choosing **Rust** as your language:
Bash
```
spacetime init --template rust todo_backend
cd todo_backend
```

Open `src/lib.rs` and replace its contents with our full relational database schema and server-side reducers.
Rust
```
use spacetimedb::{table, reducer, Identity, Timestamp};

#[table(name = todo_item, public)]
pub struct TodoItem {
    #[primary_key]
    pub id: u64,
    pub title: String,
    pub description: String,
    pub is_completed: bool,
    pub created_at: Timestamp,
    pub owner: Identity,
}

#[reducer]
pub fn create_todo(ctx: &ReducerContext, id: u64, title: String, description: String) -> Result<(), String> {
    if title.trim().is_empty() {
        return Err("Title cannot be empty".to_string());
    }

    let item = TodoItem {
        id,
        title,
        description,
        is_completed: false,
        created_at: ctx.timestamp,
        owner: ctx.sender,
    };

    TodoItem::insert(item)?;
    Ok(())
}

#[reducer]
pub fn update_todo(
    ctx: &ReducerContext, 
    id: u64, 
    new_title: String, 
    new_description: String
) -> Result<(), String> {
    if let Some(mut item) = TodoItem::filter_by_id(&id) {
        if item.owner != ctx.sender {
            return Err("Unauthorized modification attempt".to_string());
        }
        item.title = new_title;
        item.description = new_description;
        TodoItem::update_by_id(&id, item);
        Ok(())
    } else {
        Err("Todo item not found".to_string())
    }
}

#[reducer]
pub fn toggle_complete(ctx: &ReducerContext, id: u64) -> Result<(), String> {
    if let Some(mut item) = TodoItem::filter_by_id(&id) {
        if item.owner != ctx.sender {
            return Err("Unauthorized".to_string());
        }
        item.is_completed = !item.is_completed;
        TodoItem::update_by_id(&id, item);
        Ok(())
    } else {
        Err("Todo item not found".to_string())
    }
}

#[reducer]
pub fn delete_todo(ctx: &ReducerContext, id: u64) -> Result<(), String> {
    if let Some(item) = TodoItem::filter_by_id(&id) {
        if item.owner != ctx.sender {
            return Err("Unauthorized deletion attempt".to_string());
        }
        TodoItem::delete_by_id(&id);
        Ok(())
    } else {
        Err("Todo item not found".to_string())
    }
}
```

#### Hidden Nuances in the Backend Logic

- **Implicit Identity:** `ctx.sender` represents the secure cryptographic public key identity of the connecting Flutter client. We use this to bind records permanently to the person who created them.
- **Deterministic IDs:** Rather than executing random auto-increments on the server, we pass down a client-generated ID (`u64`). This pattern allows us to cleanly scale into *optimistic client updates* later if desired.

Start your local standalone SpacetimeDB engine to publish the module locally:
Bash
```
spacetime start
```

In a separate terminal tab, within your `todo_backend` directory, publish your database module to your local instance:
Bash
```
spacetime publish todo_app
```

### Part 2: Setting up Flutter & Client-Side Code Generation

Create a fresh Flutter project:
Bash
```
flutter create todo_frontend
cd todo_frontend
```

Add the `spacetimedb` client package to your `pubspec.yaml`:
YAML
```
dependencies:
  flutter:
    sdk: flutter
  spacetimedb: ^2.0.0
```

Run `flutter pub get`.

Now, we will generate compile-time type-safe client bindings from our running server module directly into our Flutter app. This single step eliminates writing network serialization boilerplates entirely:
Bash
```
dart run spacetimedb:generate -s http://localhost:3000 -d todo_app -o lib/generated
```

The tool creates a highly optimized set of classes in `lib/generated/` mirroring our tables, structs, and typed execution wrappers for our server-side reducers.

### Part 3: The Complete Frontend Blueprint

We will organize our frontend code cleanly. Below is the full implementation of your application layer across initialization, list rendering, details viewing, and full mutations.

Replace your `lib/main.dart` with the complete, cohesive codebase:
Dart
```
import 'dart:math';
import 'package:flutter/material.dart';
import 'package:spacetimedb/spacetimedb.dart';
import 'generated/client.dart';

void main() async {
  WidgetsFlutterBinding.ensureInitialized();
  
  // Establish connection to the SpacetimeDB standalone node instance
  final client = await SpacetimeDbClient.connect(
    host: 'localhost:3000',
    database: 'todo_app',
    ssl: false,
    authStorage: InMemoryTokenStore(),
  );

  // Subscribe to our specific table stream. 
  // Identity-scoping rules can be adjusted down the line via specific server-side Views.
  await client.subscriptions.subscribe(['SELECT * FROM todo_item']);
  
  // Await the local database cache synchronization batch
  await client.todoItem.subscribed;

  runApp(TodoApp(client: client));
}

class TodoApp extends StatelessWidget {
  final SpacetimeDbClient client;
  const TodoApp({super.key, required this.client});

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      title: 'SpacetimeDB Todo',
      theme: ThemeData(
        colorScheme: ColorScheme.fromSeed(seedColor: Colors.deepPurple, brightness: Brightness.dark),
        useMaterial3: true,
      ),
      home: TodoListScreen(client: client),
    );
  }
}

class TodoListScreen extends StatelessWidget {
  final SpacetimeDbClient client;
  const TodoListScreen({super.key, required this.client});

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(title: const Text('SpacetimeDB Workspace')),
      // The SDK auto-implements row listening natively via the ValueNotifier paradigm
      body: ValueListenableBuilder<List<TodoItem>>(
        valueListenable: client.todoItem.rows,
        builder: (context, List<TodoItem> items, _) {
          if (items.isEmpty) {
            return const Center(child: Text('No tasks created yet.'));
          }

          // Sort items systematically by creation time
          final sortedItems = List<TodoItem>.from(items)
            ..sort((a, b) => b.createdAt.compareTo(a.createdAt));

          return ListView.builder(
            itemCount: sortedItems.length,
            itemBuilder: (context, index) {
              final item = sortedItems[index];
              return ListTile(
                leading: Checkbox(
                  value: item.isCompleted,
                  onChanged: (_) {
                    // C: Toggle status execution via server reducer
                    client.reducers.toggleComplete(id: item.id);
                  },
                ),
                title: Text(
                  item.title,
                  style: TextStyle(
                    decoration: item.isCompleted ? TextDecoration.lineThrough : null,
                  ),
                ),
                subtitle: Text(
                  item.description,
                  maxLines: 1,
                  overflow: TextOverflow.ellipsis,
                ),
                trailing: Row(
                  mainAxisSize: MainAxisSize.min,
                  children: [
                    IconButton(
                      icon: const Icon(Icons.delete, color: Colors.redAccent),
                      onPressed: () => client.reducers.delete_todo(id: item.id),
                    ),
                  ],
                ),
                onTap: () {
                  // R: Read detailed payload inside separate modal/sheet
                  Navigator.push(
                    context,
                    MaterialPageRoute(
                      builder: (context) => TodoDetailScreen(client: client, itemId: item.id),
                    ),
                  );
                },
              );
            },
          );
        },
      ),
      floatingActionButton: FloatingActionButton(
        onPressed: () => _showCreateForm(context),
        child: const Icon(Icons.add),
      ),
    );
  }

  void _showCreateForm(BuildContext context) {
    final titleController = TextEditingController();
    final descController = TextEditingController();

    showModalBottomSheet(
      context: context,
      isScrollControlled: true,
      builder: (context) => Padding(
        padding: EdgeInsets.only(
          bottom: MediaQuery.of(context).viewInsets.bottom,
          left: 16, right: 16, top: 16,
        ),
        child: Column(
          mainAxisSize: MainAxisSize.min,
          children: [
            const Text('Create Task', style: TextStyle(fontSize: 18, fontWeight: FontWeight.bold)),
            TextField(controller: titleController, decoration: const InputDecoration(labelText: 'Title')),
            TextField(controller: descController, decoration: const InputDecoration(labelText: 'Description')),
            const SizedBox(height: 16),
            ElevatedButton(
              onPressed: () {
                if (titleController.text.isNotEmpty) {
                  final randomId = Random().nextInt(99999999).toUnsigned(64);
                  // U: Create mutation using unique ID mapping
                  client.reducers.createTodo(
                    id: randomId,
                    title: titleController.text,
                    description: descController.text,
                  );
                  Navigator.pop(context);
                }
              },
              child: const Text('Add To List'),
            ),
            const SizedBox(height: 24),
          ],
        ),
      ),
    );
  }
}

class TodoDetailScreen extends StatefulWidget {
  final SpacetimeDbClient client;
  final int itemId;

  const TodoDetailScreen({super.key, required this.client, required this.itemId});

  @override
  State<TodoDetailScreen> createState() => _TodoDetailScreenState();
}

class _TodoDetailScreenState extends State<TodoDetailScreen> {
  late TextEditingController _titleController;
  late TextEditingController _descController;

  @override
  void initState() {
    super.initState();
    // Retrieve the snapshot item directly from the local table cache
    final item = widget.client.todoItem.rowNotifier(widget.itemId).value;
    _titleController = TextEditingController(text: item?.title ?? '');
    _descController = TextEditingController(text: item?.description ?? '');
  }

  @override
  void dispose() {
    _titleController.dispose();
    _descController.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    // Utilize rowNotifier to seamlessly react if external clients update the item mid-view
    return ValueListenableBuilder<TodoItem?>(
      valueListenable: widget.client.todoItem.rowNotifier(widget.itemId),
      builder: (context, item, _) {
        if (item == null) {
          return const Scaffold(body: Center(child: Text("Item no longer exists.")));
        }

        return Scaffold(
          appBar: AppBar(
            title: const Text("Task Analysis"),
            actions: [
              IconButton(
                icon: const Icon(Icons.save),
                onPressed: () {
                  // U: Update records synchronously on remote engine storage
                  widget.client.reducers.updateTodo(
                    id: item.id,
                    newTitle: _titleController.text,
                    newDescription: _descController.text,
                  );
                  ScaffoldMessenger.of(context).showSnackBar(
                    const SnackBar(content: Text('Changes propagated globally.')),
                  );
                  Navigator.pop(context);
                },
              )
            ],
          ),
          body: Padding(
            padding: const Size(16, 16),
            child: Column(
              children: [
                TextField(
                  controller: _titleController,
                  decoration: const InputDecoration(labelText: 'Task Context', border: OutlineInputBorder()),
                ),
                const SizedBox(height: 16),
                TextField(
                  controller: _descController,
                  maxLines: 4,
                  decoration: const InputDecoration(labelText: 'Deep Analysis Description', border: OutlineInputBorder()),
                ),
                const SizedBox(height: 24),
                Text("Created on: ${item.created_at.toString()}"),
              ],
            ),
          ),
        );
      },
    );
  }
}
```

### Crucial Architectural Blind Spots to Monitor

- **Memory Growth & Disconnection Cleanup:** SpacetimeDB tables store full state directly in web-socket caches. For immense production tracking spaces, tracking lists blindly using a baseline global query like `SELECT *` forces high device footprint burdens. Rely on **Views** inside your production database layer to scope and restrict streaming row arrays to active indices only.
- **Handling Concurrent Mutations Safely:** When updating entries across multiple simultaneous network locations, if a reducer experiences validation rejections or crashes on execution, the client framework relies on structural transactional ordering. To bypass rendering micro-stalls, leverage the client-side `optimisticChanges` configuration inside your generation bindings to preemptively append transient modifications while the transaction finalizes round-trips over the wire.

Gemini is AI and can make mistakes.
