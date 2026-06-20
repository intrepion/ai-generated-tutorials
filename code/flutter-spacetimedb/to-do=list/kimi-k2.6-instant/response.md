<!--
Source: Flutter To-Do SpacetimeDB Tutorial - Kimi
URL: https://www.kimi.com/chat/19ede179-0192-8a12-8000-09bcc037d794?chat_enter_method=new_chat
Extracted: 2026-06-19
-->

Based on the latest SpacetimeDB SDK documentation for Flutter/Dart, here's a comprehensive tutorial blog post:
# Building a Real-Time To-Do List App with Flutter and SpacetimeDB
*A complete guide to CRUD operations with live sync, offline support, and optimistic updates*
## Introduction
SpacetimeDB is a database that replaces your entire backend. Instead of writing REST endpoints or GraphQL resolvers, you write Rust **reducers**—server-side functions that modify database state. Every client maintains a live, synchronized local cache, and changes propagate in real-time across all connected devices.For Flutter developers, this means you get:
- **Real-time sync** across all devices without WebSocket boilerplate
- **Optimistic updates** that make the UI feel instant
- **Offline-first support** with automatic reconnection and mutation replay
- **Type-safe APIs** generated directly from your Rust module schema

In this tutorial, we'll build a complete To-Do List app with full CRUD operations: Create, Read, Update (including completion toggles), and Delete.
## Prerequisites
Before we start, you'll need:
- **Flutter SDK** (3.0+)
- **SpacetimeDB CLI** — install with:bash    
  ```
  curl --proto '=https' --tlsv1.2 -sSf https://install.spacetimedb.com | sh
  ```
- **Rust toolchain** (for writing the server module)

## Part 1: The SpacetimeDB Server Module
First, let's build the backend. In SpacetimeDB, your backend is just a Rust crate with tables and reducers.
### 1.1 Initialize the Module
bash    
```
spacetime init --lang rust todo-server
cd todo-server
```
 
### 1.2 Define the Todo Table
Open `server/src/lib.rs` and define your data model:rust    
```
use spacetimedb::ReducerContext;

#[spacetimedb::table(
    public,
    name = "todo",
    primary_key(id)
)]
#[derive(Debug, Clone)]
pub struct Todo {
    #[auto_inc]
    pub id: u64,
    pub title: String,
    pub description: String,
    pub completed: bool,
    pub created_at: u64, // Timestamp as millis since epoch
    pub owner_id: String, // To scope todos per user/device
}
```
 Key points:
- `#[auto_inc]` automatically increments the `id` primary key
- `public` means clients can subscribe to this table
- `primary_key(id)` enables efficient row lookups and updates

### 1.3 Write the Reducers
Reducers are your server-side API endpoints. They're transactional functions that modify database state:rust    
```
// CREATE
#[spacetimedb::reducer]
pub fn create_todo(
    ctx: &ReducerContext,
    title: String,
    description: String,
    owner_id: String,
) -> Result<u64, String> {
    if title.trim().is_empty() {
        return Err("Title cannot be empty".to_string());
    }
    
    let id = ctx.db.todo().insert(Todo {
        id: 0, // auto_inc will populate this
        title,
        description,
        completed: false,
        created_at: ctx.timestamp.to_millis(),
        owner_id,
    });
    
    Ok(id)
}

// UPDATE
#[spacetimedb::reducer]
pub fn update_todo(
    ctx: &ReducerContext,
    id: u64,
    title: Option<String>,
    description: Option<String>,
) -> Result<(), String> {
    if let Some(mut todo) = ctx.db.todo().id().find(id) {
        if let Some(new_title) = title {
            if new_title.trim().is_empty() {
                return Err("Title cannot be empty".to_string());
            }
            todo.title = new_title;
        }
        if let Some(new_desc) = description {
            todo.description = new_desc;
        }
        ctx.db.todo().id().update(todo);
        Ok(())
    } else {
        Err("Todo not found".to_string())
    }
}

// TOGGLE COMPLETE STATUS
#[spacetimedb::reducer]
pub fn toggle_todo_complete(
    ctx: &ReducerContext,
    id: u64,
) -> Result<bool, String> {
    if let Some(mut todo) = ctx.db.todo().id().find(id) {
        todo.completed = !todo.completed;
        ctx.db.todo().id().update(todo);
        Ok(todo.completed)
    } else {
        Err("Todo not found".to_string())
    }
}

// DELETE
#[spacetimedb::reducer]
pub fn delete_todo(
    ctx: &ReducerContext,
    id: u64,
) -> Result<(), String> {
    if ctx.db.todo().id().delete(id) {
        Ok(())
    } else {
        Err("Todo not found".to_string())
    }
}

// READ (Server-side filtered view)
#[spacetimedb::table(
    public,
    name = "todo_view",
    primary_key(id)
)]
pub struct TodoView {
    pub id: u64,
    pub title: String,
    pub completed: bool,
}
```
 
### 1.4 Publish the Module
bash    
```
spacetime publish --server local todo-app
```
 Your backend is now live at `localhost:3000` with a real-time WebSocket API.
## Part 2: Flutter Client Setup

### 2.1 Create the Flutter Project
bash    
```
flutter create todo_flutter
cd todo_flutter
```
 
### 2.2 Add Dependencies
In `pubspec.yaml`:yaml    
```
dependencies:
  flutter:
    sdk: flutter
  spacetimedb: ^1.3.0  # Official SpacetimeDB v2 SDK
  intl: ^0.19.0        # For date formatting
  flutter_slidable: ^3.0.0  # For swipe-to-delete
```
 **Important**: Use the `spacetimedb` package on pub.dev, not the older `spacetimedb_sdk` fork. The official SDK has significantly progressed with v2 protocol support, offline storage, and modern reactive primitives. 
### 2.3 Generate Type-Safe Client Bindings
This is where the magic happens. SpacetimeDB generates Dart code directly from your Rust module:bash    
```
dart pub get
dart run spacetimedb:generate \
  -s http://localhost:3000 \
  -d todo-app \
  -o lib/generated
```
 This creates `lib/generated/client.dart` with typed classes for:
- `Todo` table rows
- `TodoView` views
- `create_todo`, `update_todo`, `toggle_todo_complete`, `delete_todo` reducers

## Part 3: Building the Flutter App

### 3.1 The SpacetimeDB Service
Create `lib/services/spacetime_service.dart` to manage the connection:dart    
```
import 'package:spacetimedb/spacetimedb.dart';
import 'package:flutter/foundation.dart';
import '../generated/client.dart';

class SpacetimeService {
  static final SpacetimeService _instance = SpacetimeService._internal();
  factory SpacetimeService() => _instance;
  SpacetimeService._internal();

  SpacetimeDbClient? _client;
  bool get isConnected => _client?.connection.isConnected ?? false;

  Future<void> initialize() async {
    // Create client with offline storage support
    _client = await SpacetimeDbClient.create(
      host: 'localhost:3000',
      database: 'todo-app',
      ssl: false,
      authStorage: InMemoryTokenStore(),
      offlineStorage: JsonFileStorage(
        basePath: '/tmp/todo_cache', // Use app docs dir in production
      ),
    );

    // Connect and subscribe to all todos
    try {
      await _client!.connect(
        initialSubscriptions: ['SELECT * FROM todo'],
      );
      
      // Wait for initial data to arrive before rendering
      await _client!.todo.subscribed;
      
      if (kDebugMode) {
        print('Connected! Todos loaded: ${_client!.todo.iter().length}');
      }
    } on SpacetimeDbException catch (e) {
      // Offline mode: cached reads still work, writes queue for later
      if (kDebugMode) print('Offline mode: $e');
    }
  }

  SpacetimeDbClient get client {
    if (_client == null) throw StateError('Service not initialized');
    return _client!;
  }

  // Connection state for UI indicators
  ValueListenable<<ConnectionState> get connectionState {
    return _client!.connection.onStateChanged;
  }
}
```
 **Key concepts here:**
- `SpacetimeDbClient.create()` loads offline cache immediately—no network needed
- `connect()` opens the WebSocket and subscribes to data
- `client.todo.subscribed` resolves when the first batch arrives, ensuring safe rendering
- Offline storage (`JsonFileStorage`) persists table snapshots and queues mutations

### 3.2 The Todo List Screen (READ)
Create `lib/screens/todo_list_screen.dart`:dart    
```
import 'package:flutter/material.dart';
import 'package:spacetimedb/spacetimedb.dart';
import '../generated/client.dart';
import '../services/spacetime_service.dart';
import 'todo_detail_screen.dart';

class TodoListScreen extends StatefulWidget {
  const TodoListScreen({super.key});

  @override
  State<TodoListScreen> createState() => _TodoListScreenState();
}

class _TodoListScreenState extends State<TodoListScreen> {
  final service = SpacetimeService();

  @override
  void initState() {
    super.initState();
    service.initialize();
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: const Text('My Todos'),
        actions: [
          // Connection status indicator
          ValueListenableBuilder<<ConnectionState>(
            valueListenable: service.connectionState,
            builder: (context, state, _) {
              final color = switch (state) {
                ConnectionState.connected => Colors.green,
                ConnectionState.connecting => Colors.orange,
                ConnectionState.reconnecting => Colors.orange,
                _ => Colors.red,
              };
              return Padding(
                padding: const EdgeInsets.all(16.0),
                child: Icon(Icons.circle, color: color, size: 12),
              );
            },
          ),
        ],
      ),
      // The core reactive pattern: ValueListenableBuilder on the table cache
      body: ValueListenableBuilder<List<Todo>>(
        valueListenable: service.client.todo.rows,
        builder: (context, todos, _) {
          if (todos.isEmpty) {
            return const Center(
              child: Text('No todos yet. Create one!'),
            );
          }

          // Sort by creation date, newest first
          final sorted = todos.toList()
            ..sort((a, b) => b.created_at.compareTo(a.created_at));

          return ListView.builder(
            itemCount: sorted.length,
            itemBuilder: (context, index) {
              final todo = sorted[index];
              return TodoListItem(todo: todo);
            },
          );
        },
      ),
      floatingActionButton: FloatingActionButton(
        onPressed: () => _showCreateDialog(context),
        child: const Icon(Icons.add),
      ),
    );
  }

  void _showCreateDialog(BuildContext context) {
    showDialog(
      context: context,
      builder: (context) => const CreateTodoDialog(),
    );
  }
}
```
 **Why `ValueListenableBuilder`?** The SpacetimeDB SDK exposes table data as `ValueNotifier<List<Todo>>`. This is *held state*—new subscribers immediately see the current value. It rebuilds automatically whenever any client (including you on another device) modifies the data. 
### 3.3 The Todo List Item (UPDATE + DELETE)
Create `lib/widgets/todo_list_item.dart`:dart    
```
import 'package:flutter/material.dart';
import 'package:flutter_slidable/flutter_slidable.dart';
import '../generated/client.dart';
import '../services/spacetime_service.dart';
import '../screens/todo_detail_screen.dart';

class TodoListItem extends StatelessWidget {
  final Todo todo;

  const TodoListItem({super.key, required this.todo});

  @override
  Widget build(BuildContext context) {
    final service = SpacetimeService();

    return Slidable(
      // Swipe right to delete
      endActionPane: ActionPane(
        motion: const ScrollMotion(),
        children: [
          SlidableAction(
            onPressed: (_) => _deleteTodo(context),
            backgroundColor: Colors.red,
            foregroundColor: Colors.white,
            icon: Icons.delete,
            label: 'Delete',
          ),
        ],
      ),
      child: ListTile(
        leading: Checkbox(
          value: todo.completed,
          // Toggle completion status
          onChanged: (_) => _toggleComplete(service),
        ),
        title: Text(
          todo.title,
          style: TextStyle(
            decoration: todo.completed ? TextDecoration.lineThrough : null,
            color: todo.completed ? Colors.grey : null,
          ),
        ),
        subtitle: Text(
          todo.description,
          maxLines: 1,
          overflow: TextOverflow.ellipsis,
        ),
        trailing: Text(
          _formatDate(todo.created_at),
          style: Theme.of(context).textTheme.bodySmall,
        ),
        onTap: () => _navigateToDetail(context),
      ),
    );
  }

  Future<void> _toggleComplete(SpacetimeService service) async {
    try {
      // Call the reducer - this updates locally via optimistic update
      // then confirms with server. If rejected, auto-rolls back.
      final result = await service.client.reducers.toggleTodoComplete(
        id: todo.id,
      );
      
      if (result.isSuccess) {
        final newStatus = result.value; // true/false from server
        ScaffoldMessenger.of(context).showSnackBar(
          SnackBar(
            content: Text(newStatus ? 'Completed!' : 'Marked incomplete'),
            duration: const Duration(seconds: 1),
          ),
        );
      }
    } on SpacetimeDbException catch (e) {
      ScaffoldMessenger.of(context).showSnackBar(
        SnackBar(content: Text('Error: $e')),
      );
    }
  }

  Future<void> _deleteTodo(BuildContext context) async {
    final confirmed = await showDialog<bool>(
      context: context,
      builder: (context) => AlertDialog(
        title: const Text('Delete Todo?'),
        content: Text('Are you sure you want to delete "${todo.title}"?'),
        actions: [
          TextButton(
            onPressed: () => Navigator.pop(context, false),
            child: const Text('Cancel'),
          ),
          TextButton(
            onPressed: () => Navigator.pop(context, true),
            child: const Text('Delete', style: TextStyle(color: Colors.red)),
          ),
        ],
      ),
    );

    if (confirmed == true) {
      try {
        final service = SpacetimeService();
        final result = await service.client.reducers.deleteTodo(id: todo.id);
        
        if (result.isSuccess) {
          ScaffoldMessenger.of(context).showSnackBar(
            const SnackBar(content: Text('Todo deleted')),
          );
        }
      } on SpacetimeDbException catch (e) {
        ScaffoldMessenger.of(context).showSnackBar(
          SnackBar(content: Text('Error: $e')),
        );
      }
    }
  }

  void _navigateToDetail(BuildContext context) {
    Navigator.push(
      context,
      MaterialPageRoute(
        builder: (context) => TodoDetailScreen(todoId: todo.id),
      ),
    );
  }

  String _formatDate(int millis) {
    final date = DateTime.fromMillisecondsSinceEpoch(millis);
    return '${date.month}/${date.day}/${date.year}';
  }
}
```
 **Optimistic updates explained:** When you call `toggleTodoComplete`, the SDK immediately applies the change to the local cache. The UI updates instantly. The server then confirms (or rejects) the change. If rejected, the SDK automatically rolls back the local state. The user sees instant feedback without waiting for the network round-trip. 
### 3.4 Create Todo Dialog (CREATE)
Create `lib/widgets/create_todo_dialog.dart`:dart    
```
import 'package:flutter/material.dart';
import '../services/spacetime_service.dart';

class CreateTodoDialog extends StatefulWidget {
  const CreateTodoDialog({super.key});

  @override
  State<<CreateTodoDialog> createState() => _CreateTodoDialogState();
}

class _CreateTodoDialogState extends State<<CreateTodoDialog> {
  final _titleController = TextEditingController();
  final _descriptionController = TextEditingController();
  final _formKey = GlobalKey<<FormState>();
  bool _isLoading = false;

  @override
  void dispose() {
    _titleController.dispose();
    _descriptionController.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    return AlertDialog(
      title: const Text('New Todo'),
      content: Form(
        key: _formKey,
        child: Column(
          mainAxisSize: MainAxisSize.min,
          children: [
            TextFormField(
              controller: _titleController,
              decoration: const InputDecoration(
                labelText: 'Title',
                hintText: 'What needs to be done?',
              ),
              validator: (value) {
                if (value == null || value.trim().isEmpty) {
                  return 'Title is required';
                }
                return null;
              },
              autofocus: true,
            ),
            const SizedBox(height: 16),
            TextFormField(
              controller: _descriptionController,
              decoration: const InputDecoration(
                labelText: 'Description (optional)',
                hintText: 'Add details...',
              ),
              maxLines: 3,
            ),
          ],
        ),
      ),
      actions: [
        TextButton(
          onPressed: () => Navigator.pop(context),
          child: const Text('Cancel'),
        ),
        ElevatedButton(
          onPressed: _isLoading ? null : _submit,
          child: _isLoading
              ? const SizedBox(
                  width: 20,
                  height: 20,
                  child: CircularProgressIndicator(strokeWidth: 2),
                )
              : const Text('Create'),
        ),
      ],
    );
  }

  Future<void> _submit() async {
    if (!_formKey.currentState!.validate()) return;

    setState(() => _isLoading = true);

    try {
      final service = SpacetimeService();
      final result = await service.client.reducers.createTodo(
        title: _titleController.text.trim(),
        description: _descriptionController.text.trim(),
        ownerId: service.client.identity?.toHexString() ?? 'anonymous',
      );

      if (result.isSuccess) {
        final newId = result.value;
        Navigator.pop(context);
        ScaffoldMessenger.of(context).showSnackBar(
          SnackBar(content: Text('Todo created with ID: $newId')),
        );
      }
    } on SpacetimeDbException catch (e) {
      ScaffoldMessenger.of(context).showSnackBar(
        SnackBar(content: Text('Error: $e')),
      );
    } finally {
      setState(() => _isLoading = false);
    }
  }
}
```
 
### 3.5 Todo Detail Screen (READ single item + UPDATE)
Create `lib/screens/todo_detail_screen.dart`:dart    
```
import 'package:flutter/material.dart';
import 'package:spacetimedb/spacetimedb.dart';
import '../generated/client.dart';
import '../services/spacetime_service.dart';

class TodoDetailScreen extends StatelessWidget {
  final int todoId;

  const TodoDetailScreen({super.key, required this.todoId});

  @override
  Widget build(BuildContext context) {
    final service = SpacetimeService();

    return Scaffold(
      appBar: AppBar(
        title: const Text('Todo Details'),
        actions: [
          IconButton(
            icon: const Icon(Icons.edit),
            onPressed: () => _showEditDialog(context, service),
          ),
        ],
      ),
      // Watch only THIS specific row - O(1) updates, not O(table size)
      body: ValueListenableBuilder<Todo?>(
        valueListenable: service.client.todo.rowNotifier(todoId),
        builder: (context, todo, _) {
          if (todo == null) {
            return const Center(child: Text('Todo not found'));
          }

          return Padding(
            padding: const EdgeInsets.all(16.0),
            child: Column(
              crossAxisAlignment: CrossAxisAlignment.start,
              children: [
                Row(
                  children: [
                    Chip(
                      label: Text(
                        todo.completed ? 'Completed' : 'Pending',
                        style: TextStyle(
                          color: todo.completed ? Colors.green : Colors.orange,
                        ),
                      ),
                      backgroundColor: todo.completed
                          ? Colors.green.withOpacity(0.1)
                          : Colors.orange.withOpacity(0.1),
                    ),
                    const Spacer(),
                    Text(
                      'ID: ${todo.id}',
                      style: Theme.of(context).textTheme.bodySmall,
                    ),
                  ],
                ),
                const SizedBox(height: 24),
                Text(
                  todo.title,
                  style: Theme.of(context).textTheme.headlineSmall,
                ),
                const SizedBox(height: 16),
                if (todo.description.isNotEmpty) ...[
                  Text(
                    'Description',
                    style: Theme.of(context).textTheme.titleMedium,
                  ),
                  const SizedBox(height: 8),
                  Text(todo.description),
                  const SizedBox(height: 24),
                ],
                Text(
                  'Created: ${_formatDate(todo.created_at)}',
                  style: Theme.of(context).textTheme.bodySmall,
                ),
                const SizedBox(height: 8),
                Text(
                  'Owner: ${todo.owner_id.substring(0, 8)}...',
                  style: Theme.of(context).textTheme.bodySmall,
                ),
                const Spacer(),
                SizedBox(
                  width: double.infinity,
                  child: ElevatedButton.icon(
                    onPressed: () => _toggleComplete(context, service, todo),
                    icon: Icon(
                      todo.completed ? Icons.undo : Icons.check_circle,
                    ),
                    label: Text(
                      todo.completed ? 'Mark Incomplete' : 'Mark Complete',
                    ),
                    style: ElevatedButton.styleFrom(
                      backgroundColor:
                          todo.completed ? Colors.orange : Colors.green,
                      foregroundColor: Colors.white,
                    ),
                  ),
                ),
              ],
            ),
          );
        },
      ),
    );
  }

  Future<void> _toggleComplete(
    BuildContext context,
    SpacetimeService service,
    Todo todo,
  ) async {
    try {
      await service.client.reducers.toggleTodoComplete(id: todo.id);
    } on SpacetimeDbException catch (e) {
      ScaffoldMessenger.of(context).showSnackBar(
        SnackBar(content: Text('Error: $e')),
      );
    }
  }

  void _showEditDialog(BuildContext context, SpacetimeService service) {
    // Get current value for the dialog
    final currentTodo = service.client.todo.rowNotifier(todoId).value;
    if (currentTodo == null) return;

    showDialog(
      context: context,
      builder: (context) => EditTodoDialog(todo: currentTodo),
    );
  }

  String _formatDate(int millis) {
    final date = DateTime.fromMillisecondsSinceEpoch(millis);
    return '${date.month}/${date.day}/${date.year} ${date.hour}:${date.minute.toString().padLeft(2, '0')}';
  }
}
```
 **Performance note:** `rowNotifier(id)` watches only that specific row. If you have 1000 todos and only one changes, only that one widget rebuilds. This is O(1) per transaction, not O(listeners × events). 
### 3.6 Edit Todo Dialog (UPDATE)
Create `lib/widgets/edit_todo_dialog.dart`:dart    
```
import 'package:flutter/material.dart';
import '../generated/client.dart';
import '../services/spacetime_service.dart';

class EditTodoDialog extends StatefulWidget {
  final Todo todo;

  const EditTodoDialog({super.key, required this.todo});

  @override
  State<<EditTodoDialog> createState() => _EditTodoDialogState();
}

class _EditTodoDialogState extends State<<EditTodoDialog> {
  late final TextEditingController _titleController;
  late final TextEditingController _descriptionController;
  final _formKey = GlobalKey<<FormState>();
  bool _isLoading = false;

  @override
  void initState() {
    super.initState();
    _titleController = TextEditingController(text: widget.todo.title);
    _descriptionController = TextEditingController(text: widget.todo.description);
  }

  @override
  void dispose() {
    _titleController.dispose();
    _descriptionController.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    return AlertDialog(
      title: const Text('Edit Todo'),
      content: Form(
        key: _formKey,
        child: Column(
          mainAxisSize: MainAxisSize.min,
          children: [
            TextFormField(
              controller: _titleController,
              decoration: const InputDecoration(labelText: 'Title'),
              validator: (value) {
                if (value == null || value.trim().isEmpty) {
                  return 'Title is required';
                }
                return null;
              },
            ),
            const SizedBox(height: 16),
            TextFormField(
              controller: _descriptionController,
              decoration: const InputDecoration(labelText: 'Description'),
              maxLines: 3,
            ),
          ],
        ),
      ),
      actions: [
        TextButton(
          onPressed: () => Navigator.pop(context),
          child: const Text('Cancel'),
        ),
        ElevatedButton(
          onPressed: _isLoading ? null : _submit,
          child: _isLoading
              ? const SizedBox(
                  width: 20,
                  height: 20,
                  child: CircularProgressIndicator(strokeWidth: 2),
                )
              : const Text('Save'),
        ),
      ],
    );
  }

  Future<void> _submit() async {
    if (!_formKey.currentState!.validate()) return;

    setState(() => _isLoading = true);

    try {
      final service = SpacetimeService();
      final result = await service.client.reducers.updateTodo(
        id: widget.todo.id,
        title: _titleController.text.trim(),
        description: _descriptionController.text.trim(),
      );

      if (result.isSuccess) {
        Navigator.pop(context);
        ScaffoldMessenger.of(context).showSnackBar(
          const SnackBar(content: Text('Todo updated successfully')),
        );
      }
    } on SpacetimeDbException catch (e) {
      ScaffoldMessenger.of(context).showSnackBar(
        SnackBar(content: Text('Error: $e')),
      );
    } finally {
      setState(() => _isLoading = false);
    }
  }
}
```
 
## Part 4: Advanced Patterns

### 4.1 Listening to Specific Events
Sometimes you need to react to transient events (like showing a toast when *someone else* creates a todo). Use Streams for this:dart    
```
// In your service initialization:
service.client.todo.onInsert.listen((event) {
  final context = event.context;
  
  // Skip if this was our own transaction (we already showed feedback)
  if (context.isMyTransaction) return;
  
  // Someone else added a todo!
  final todo = event.newRow;
  showNotification('New todo from ${todo.owner_id}: ${todo.title}');
});
```
 Streams are for *transient events*—late subscribers miss past events. ValueNotifiers are for *held state*—always has a current value. 
### 4.2 Filtering with Server-Side Views
For large datasets, create server-side views instead of filtering client-side:rust    
```
// In server/src/lib.rs
#[spacetimedb::view(
    public,
    name = "completed_todos",
    sql = "SELECT * FROM todo WHERE completed = true"
)]
pub struct CompletedTodos;
```
 Then subscribe to just that view:dart    
```
await client.connect(
  initialSubscriptions: ['SELECT * FROM completed_todos'],
);
```
 
### 4.3 Handling Offline Mode
The SDK handles offline gracefully. When you call a reducer while disconnected:
- The change applies optimistically to local cache (UI updates)
- The reducer call queues locally
- On reconnect, queued calls replay automatically
- If the server rejects, the optimistic change rolls back

dart    
```
// Check pending mutations
print('Pending: ${client.syncState.pendingCount}');

// Listen for connection state
client.connection.onStateChanged.addListener(() {
  final state = client.connection.onStateChanged.value;
  if (state == ConnectionState.connected) {
    hideOfflineBanner();
  } else {
    showOfflineBanner();
  }
});
```
 
## Part 5: Running the App

### 5.1 Start the Server
bash    
```
cd todo-server
spacetime publish --server local todo-app
```
 
### 5.2 Run the Flutter App
bash    
```
cd todo_flutter
flutter run
```
 
### 5.3 Test Real-Time Sync

- Open the app on your phone/emulator
- Open a second instance (web or another device)
- Create a todo in one — watch it appear instantly in the other
- Toggle completion — both screens update simultaneously
- Turn on airplane mode on one device, make changes, then reconnect — changes sync automatically

## Summary
You now have a fully functional, real-time To-Do app with:Table     FeatureImplementation **Create**`createTodo` reducer + dialog form**Read (List)**`ValueListenableBuilder` on `client.todo.rows`**Read (Single)**`rowNotifier(id)` for O(1) per-row updates**Update**`updateTodo` reducer + edit dialog**Complete Toggle**`toggleTodoComplete` reducer with optimistic UI**Delete**`deleteTodo` reducer + swipe-to-delete**Real-time Sync**Automatic via WebSocket + table cache**Offline Support**`JsonFileStorage` + optimistic mutation queue
### Key Takeaways

- **No REST APIs needed** — Reducers are your backend endpoints
- **Reactive by default** — `ValueListenableBuilder` + `TableCache` handles all sync
- **Optimistic updates** — UI feels instant, server validates asynchronously
- **Type safety** — Generated code from Rust schema eliminates runtime errors
- **Offline-first** — Works without network, syncs when available

SpacetimeDB eliminates the traditional frontend-backend glue code. Your Flutter app talks directly to the database as if it were local, while SpacetimeDB handles synchronization, persistence, and conflict resolution behind the scenes.
## Further Reading

- SpacetimeDB Dart SDK Documentation
- SpacetimeDB Official Docs
- Chat App Tutorial (TypeScript) — patterns translate directly to Dart

*Happy coding! Your todos now sync across the universe in real-time.* 🚀
