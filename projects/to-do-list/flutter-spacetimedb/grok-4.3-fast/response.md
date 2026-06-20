<!--
Source: Flutter To-Do App with SpacetimeDB - Grok
URL: https://grok.com/c/6d4b9b5c-7af8-44e3-9af2-3da838384e66?rid=aef6cb89-45b9-4e5b-bdd1-2e8ebb0a1951
Extracted: 2026-06-19
-->

**Building a To-Do List App with Flutter and SpacetimeDB**⁠Pub

### Introduction

Simple real-time To-Do app using Flutter and SpacetimeDB for backend/database with live sync.

### Prerequisites

- Flutter SDK
- SpacetimeDB CLI (spacetime installed)
- Dart package: spacetimedb: ^1.3.0

### 1. Setup SpacetimeDB Module

Create a module (e.g., Rust/TS) with Todo table:

SQL
```
CREATE TABLE todo (
  id IDENTITY PRIMARY KEY,
  title TEXT NOT NULL,
  completed BOOLEAN DEFAULT false,
  created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);
```

Add reducers for CRUD (insert, update, delete). Publish: spacetime publish your_db.

### 2. Flutter Project Setup

Bash
```
flutter create todo_app
cd todo_app
dart pub add spacetimedb
```

Generate client:

Bash
```
dart run spacetimedb:generate -s http://localhost:3000 -d your_db -o lib/generated
```

### 3. Main App & Connection

Dart
```
// lib/main.dart
import 'package:flutter/material.dart';
import 'package:spacetimedb/spacetimedb.dart';
import 'generated/client.dart'; // Generated

void main() async {
  WidgetsFlutterBinding.ensureInitialized();
  final client = await SpacetimeDbClient.connect(
    host: 'localhost:3000',
    database: 'your_db',
    ssl: false,
    authStorage: InMemoryTokenStore(),
    initialSubscriptions: ['SELECT * FROM todo'],
  );
  runApp(MyApp(client: client));
}

class MyApp extends StatelessWidget {
  final SpacetimeDbClient client;
  // ...
}
```

### 4. UI & Operations (HomeScreen)

Use ListView.builder with client.todo.iter() for list.⁠Pub

- **Show List**: client.todo.iter() + ValueListenableBuilder or streams for live updates.
- **Create**:Dart
  ```
  await client.reducers.createTodo(title: 'New task'); // Via reducer
  ```
- **Read**: client.todo.iter().firstWhere((t) => t.id == id);
- **Update**:Dart
  ```
  await client.reducers.updateTodo(id: id, title: newTitle);
  ```
- **Toggle Complete**:Dart
  ```
  await client.reducers.toggleComplete(id: id);
  ```
- **Delete**:Dart
  ```
  await client.reducers.deleteTodo(id: id);
  ```

Listen to streams:

Dart
```
client.todo.insertStream.listen((todo) => setState(() {}));
client.todo.updateStream.listen((update) => setState(() {}));
client.todo.deleteStream.listen((id) => setState(() {}));
```

### 5. Basic UI Sketch

- AppBar: "To-Dos"
- FloatingActionButton: Add new
- ListTiles with Checkbox for toggle, edit/delete icons.

This covers full CRUD with real-time sync. Test locally, deploy to SpacetimeDB cloud. Expand with auth/offline support.
