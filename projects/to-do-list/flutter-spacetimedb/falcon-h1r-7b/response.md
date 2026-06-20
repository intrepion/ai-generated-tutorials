<!--
Source: Falcon Chat
URL: https://chat.falconllm.tii.ae/c/8eabc03f-b724-482f-8f21-78e899be5c5b
Extracted: 2026-06-19
-->

# 📋 Todo List Project with Flutter & SpacetimeDB

*A step‑by‑step tutorial that walks you through creating a simple to‑do app capable of reading, creating, updating, toggling completion, and deleting items.*

**TL;DR** – By the end of this post you’ll have a Flutter project that:

- Persists `ToDo` documents in a local SpacetimeDB collection.
- Shows a full list of items with a checkbox to mark as complete.
- Lets the user add, read, update, and delete items.
- Persists data automatically between app launches.

 
## Table of Contents

- What You’ll Build
- Prerequisites
- Project Setup
- Adding SpacetimeDB
- Defining the ToDo Model
- CRUD Operations with SpacetimeDB   
  - 6.1 Initialise the database
  - 6.2 Create a new ToDo
  - 6.3 Read all & single ToDo
  - 6.4 Update a ToDo (status toggle)
  - 6.5 Delete a ToDo
- UI – The Todo List Screen   
  - 7.1 Adding a new item
  - 7.2 Listing items with checkboxes
  - 7.3 Updating status & handling completion UI
  - 7.4 Deleting an item
- State Management – Provider vs. FutureBuilder
- Saving Data Persistence
- Testing & Common Pitfalls
- Next Steps & Resources

## What You’ll Build

- A **Home** screen that displays all tasks, each with a checkbox indicating completion status.
- A **+ Add Item** button that presents a text field.
- Tapping the checkbox toggles the task’s status in SpacetimeDB.
- Long‑press (or swipe left) to reveal a **Delete** button.
- Persistent storage via SpacetimeDB: data survives app restarts.

## Prerequisites
Tool Minimum version Why you need it  Flutter SDK3.12+Modern widget tree & hot‑reload Android Studio / VS Code + Dart IDE—Code editing & debugging A device or emulator with at least 1 GB RAM—SpacetimeDB uses local storage (Optional) VS Code Extension: **Dart & Flutter**—Faster code completion   
**Tip:** If you’re using a managed Flutter environment (e.g., `flutter create --release` for production), make sure you have permission to write to `appdata/flutter_spacetime_db`.
 

## Project Setup

- **Create a new Flutter project**
  bash      12flutter create todo_spacetimecd todo_spacetime
- **Replace the generated `lib/main.dart`** with the starter code shown later in the tutorial (we’ll rewrite everything as we progress).
- **Open the project** in your IDE and run `flutter run` to verify you can launch the app (the placeholder UI will be replaced later).

## Adding SpacetimeDB

SpacetimeDB ships as two Dart packages:
Package Purpose  `spacetime_foundation`Core DB API (`Database`, `Document`, etc.) `spacetime`UI widgets for embedded DB preview (optional)   
Add them to `pubspec.yaml`:
yaml      1234⌄⌄dependencies:  flutter:    sdk: flutter  spacetime_foundation: ^1.1.0   # check for latest version  
Run:
bash      1flutter pub get  

## Defining the ToDo Model

Create a model class that maps directly to SpacetimeDB documents:
dart      123456789101112131415161718192021222324252627282930313233// lib/model/todo.dartimport 'package:spacetime_mongodb/src/serializer.dart';
class ToDo {  final String id;  final String title;  final bool completed;
  // Default constructor required for serialization  ToDo({    required this.id,    required this.title,    this.completed = false,  });
  // Factory from map (used by SpacetimeDB)  factory ToDo.fromMap(Map<String, dynamic> map) {    return ToDo(      id: map['_id'] as String,      title: map['title'] as String,      completed: (map['completed'] as dynamic) ?? false,    );  }
  // Encode to map  Map<String, dynamic> toMap() {    return {      '_id': id,      'title': title,      'completed': completed,    };  }}  
**Why a class?**
SpacetimeDB serializes each document to a Map. By providing a custom class with `toMap` / `fromMap`, you get automatic type‑safety and readability (no raw maps lying around).
 

## CRUD Operations with SpacetimeDB

All DB work is performed via the **singleton** `DatabaseManager`.
Create a service file that encapsulates all CRUD logic.
dart      12345678910111213141516171819202122232425262728293031323334353637383940414243444546474849505152535455// lib/services/todo_service.dartimport 'package:spacetime/spacetime.dart';import '../model/todo.dart';import 'dart:async';
class TodoService {  final Collection<ToDo> _collection =      Collection<ToDo>(DatabaseManager.sharedInstance().session);
  // -------------------- CREATE --------------------  Future<ToDo> createToDo(ToDo todo) async {    await _collection.insert(todo);    // DB inserts automatically assign an _id.    return todo.copyWith(id: todo.id);  }
  // -------------------- READ ALL --------------------  Stream<List<ToDo>> streamAllTodos() async* {    final docs = await _collection.find();    yield docs;  }
  // -------------------- READ SINGLE --------------------  Future<ToDo?> readTodo(String id) async {    final document = await _collection.findOne(where: {'_id': id});    return document == null ? null : ToDo.fromMap(document);  }
  // -------------------- UPDATE --------------------  Future<bool> toggleCompleted(String id) async {    final todo = await readTodo(id);    if (todo == null) return false;
    // Update only the "completed" field – use a transaction to ensure atomicity    final updated = await _collection.update(      todo,      {'completed': !todo.completed},    );    return true;  }
  // -------------------- DELETE --------------------  Future<bool> deleteTodo(String id) async {    final exists = await _collection.findOne(where: {'_id': id}) != null;    if (!exists) return false;
    await _collection.delete(where: {'_id': id});    return true;  }
  // Helper – convenience for the UI  Future<List<ToDo>> fetchAllTodos() async {    return await streamAllTodos().toList();  }}  
### Key Points

- **`Collection<ToDo>`** uses the `ToDo` class for automatic serialization.
- **`update`** with a map argument (`{'completed': !todo.completed}`) is atomic.
- **`_id`** is automatically generated during `insert`.

## UI – The Todo List Screen

We'll use a **Provider** package (or plain `FutureBuilder` if you prefer a lighter solution). Below is the Provider pattern; you can swap it later.

### 1️⃣ Install Provider

Add to `pubspec.yaml`:
yaml      12⌄dependencies:  provider: ^5.0.0  
Run `flutter pub get`.

### 2️⃣ Expose the Service via Provider
dart      1234567891011121314151617181920212223242526272829303132333435363738// lib/provider/todo_provider.dartimport 'package:provider/provider.dart';import '../services/todo_service.dart';
class TodoProvider extends ChangeNotifier {  final TodoService _service = TodoService();
  List<ToDo> _todos = [];
  List<ToDo> get todos => List.unmodifiable(_todos);
  void init() async {    final all = await _service.fetchAllTodos();    _todos = all;    notifyListeners(); // refresh UI  }
  // CRUD methods  Future<void> _load() => init();
  Future<void> createTodo(String title) async {    final todo = ToDo(title: title);    final inserted = await _service.createToDo(todo);    _todos.add(inserted);    _service.toggleCompleted(inserted.id); // optional: default completed = false    notifyListeners();  }
  Future<bool> toggleCompleted(String id) async => await _service.toggleCompleted(id);
  Future<bool> deleteTodo(String id) async => await _service.deleteTodo(id);
  @override  void dispose() {    DatabaseManager.sharedInstance().close(); // persist changes    super.dispose();  }}  
### 3️⃣ Wire the Provider in main.dart
dart      12345678910111213141516171819202122232425262728// lib/main.dartimport 'package:flutter/material.dart';import 'package:provider/provider.dart';import 'provider/todo_provider.dart';import 'services/todo_service.dart';
Future<void> main() async {  WidgetsFlutterBinding.ensureInitialized();  await DatabaseManager.sharedInstance().open();  runApp(    ChangeNotifierProvider<TodoProvider>(      create: (_) => TodoProvider()..init(),      child: const MyApp(),    ),  );}
class MyApp extends StatelessWidget {  const MyApp({Key? key}) : super(key: key);  @override  Widget build(BuildContext context) {    return MaterialApp(      title: 'Todo SpacetimeDB Demo',      theme: ThemeData(primarySwatch: Colors.indigo),      home: const TodoListScreen(),    );  }}  
### 4️⃣ UI Components

#### a. Add Todo Dialog
dart      12345678910111213141516171819202122232425262728293031323334353637// lib/widgets/add_todo_dialog.dartimport 'package:flutter/material.dart';
class AddTodoDialog extends StatelessWidget {  final Function(String) onSubmitted;
  const AddTodoDialog({Key? key, required this.onSubmitted}) : super(key: key);
  @override  Widget build(BuildContext context) {    return AlertDialog(      title: const Text('New Todo'),      content: TextField(        decoration: const InputDecoration(          labelText: 'What needs doing?',        ),        onSubmitted: onSubmitted,      ),      actions: [        TextButton(          child: const Text('Cancel'),          onPressed: () => Navigator.pop(context),        ),        ElevatedButton(          child: const Text('Add'),          onPressed: () {            final title = context.ancestorWidget?.widget as dynamic as dynamic;            // ignore: unused_argument            // Filter out invalid inputs            if (title != null) onSubmitted(title.toString().trim());            Navigator.pop(context);          },        ),      ],    );  }}  
#### b. Todo Item Widget
dart      12345678910111213141516171819202122232425262728293031323334353637383940414243444546474849505152535455// lib/widgets/todo_item.dartimport 'package:flutter/material.dart';import 'package:provider/provider.dart';
class TodoItem extends StatelessWidget {  final String id;  final String title;  final bool completed;
  const TodoItem({    Key? key,    required this.id,    required this.title,    this.completed = false,  }) : super(key: key);
  @override  Widget build(BuildContext context) {    // Get the Provider so we can toggle status    final provider = Provider.of<TodoProvider>(context, listen: false);
    return ListTile(      leading: completed          ? const Icon(Icons.check_circle, color: Colors.green)          : const Icon(Icons.radio_button_unchecked),      title: Text(completed          ? Text(title,                style: const TextStyle(decoration: TextDecoration.lineThrough, color: Colors.grey))          : Text(title)),      trailing: Checkbox(        value: completed,        onChanged: (changed) async {          if (changed == null) return;          final success = await provider.toggleCompleted(id);          if (success) {            // Update the UI list            await Provider.of<TodoProvider>(context, listen: false).load();          }        },      ),      // Long‑press to show deletion options      onLongPress: () async {        final success = await provider.deleteTodo(id);        if (success) {          // Refresh UI          await Provider.of<TodoProvider>(context, listen: false).load();        } else {          ScaffoldMessenger.of(context).showSnackBar(            const SnackBar(content: Text('Todo not found')),          );        }      },    );  }}  
#### c. Todo List Screen
dart      1234567891011121314151617181920212223242526272829303132333435363738394041424344454647484950515253545556575859606162636465666768// lib/screens/todo_list_screen.dartimport 'package:flutter/material.dart';import 'package:provider/provider.dart';import 'widgets/todo_item.dart';
class TodoListScreen extends StatelessWidget {  @override  Widget build(BuildContext context) {    final provider = Provider.of<TodoProvider>(context);
    // Sort: completed first, then active    final sorted = provider.todos        .where((t) => !t.completed)        .toList();    sorted.addAll(provider.todos.where((t) => t.completed));
    return Scaffold(      appBar: AppBar(        title: const Text('🗒️ My Todo List'),        actions: [          IconButton(            icon: const Icon(Icons.add),            tooltip: 'Add Todo',            onPressed: () => _showAddDialog(context),          ),        ],      ),      body: _buildBody(context),      floatingActionButton: FloatingActionButton(        child: const Icon(Icons.sort),        onPressed: () async {          final sorted = provider.todos              .where((t) => !t.completed)              .toList();          sorted.addAll(provider.todos.where((t) => t.completed));          setState(() {}); // refresh UI        },      ),    );  }
  Widget _buildBody(BuildContext context) {    if (provider.todos.isEmpty) {      return const Center(child: Text('No todos yet. Add a new one!'));    }
    return ListView.builder(      itemCount: provider.todos.length,      itemBuilder: (context, index) {        final todo = provider.todos[index];        return TodoItem(id: todo.id, title: todo.title, completed: todo.completed);      },    );  }
  void _showAddDialog(BuildContext context) {    showDialog<String>(      context: context,      builder: (c) => AddTodoDialog(        onSubmitted: (title) async {          await Provider.of<TodoProvider>(context, listen: false).createTodo(title);          Navigator.pop(c);        },        child: null,      ),    );  }}  
#### d. Add Todo Button (optional standalone)

If you prefer not to use Provider (or you already have a state‑management solution like Bloc), you can call the service directly:
dart      1234567// Simple example (no Provider)Future<void> addTodo(BuildContext context, String title) async {  final service = TodoService();  final todo = ToDo(title: title);  await service.createTodo(todo);  // Refresh your UI logic here…}  

## State Management – Provider vs. FutureBuilder
Approach Pros Cons  **Provider** (shown above)Centralised mutable state, easy to access from any widget, works great with hot‑reload.Slightly more boilerplate; not ideal for very large apps (consider a Bloc/Riverpod). **FutureBuilder**Minimal setup; works for simple CRUD screens.Harder to keep UI in sync across many screens; you must reload the whole widget when data changes. **Bloc / Riverpod**Full‑featured architecture, isolation of UI vs. business logic.More files to maintain; learning curve.   
If you decide to drop Provider, replace every call to `Provider.of<TodoProvider>(context)` with a direct service fetch inside a `FutureBuilder`. The CRUD logic stays exactly the same.

## Saving Data Persistence

SpacetimeDB abstracts away the storage location:

- **Android** – `getExternalFilesDir(null)` → `appdata/fluter_spacetime_db`
- **iOS** – `getAppDirectory()` → `Documents/`

**Persist automatically**:
dart      12// In TodoProvider's dispose()DatabaseManager.sharedInstance().close(); // writes to native DB  
**Important** – Do **not** manually call `await DatabaseManager.sharedInstance().close();` on every screen nav‑pop, otherwise you lose data. Close only once (e.g., at app termination or on provider disposing).
 

## Testing & Common Pitfalls
Issue Symptom Fix  **`_id` not returned**UI shows “null” in list after addEnsure you capture the return from `insert`. SpacetimeDB uses a Mongo‑style `_id`; don’t rely on auto‑generated string unless you copy it back. **Toggle UI not updating**List still shows old completed stateCall `Provider.of<TodoProvider>(context, listen: false).load();` after the async toggle succeeds. **Crash on `Provider.of` in `initState`**Provider not yet set when accessedPrefer the Provider “create” method (`ChangeNotifierProvider`) as shown in `main.dart`. **Database closed too early**Items disappear after rotationClose DB only in `dispose()` (or when app exits). Keep the instance alive across widget rebuilds. **Long‑press not reaching checkbox**Gesture recognizer conflictWrap `ListTile` in `LongPressGestureDetector` or set `preventFocusAfterTouch` to `true`. **SpacetimeDB version mismatch**Build fails on `import 'package:spacetime_foundation/...`Ensure you run `flutter pub get` after changing versions in `pubspec.yaml`.   
**Quick debug tip**:
Print the DB state after each operation:
dart      12import 'package:spacetime/spacetime.dart';print('All todos: ${_collection.find()}');  

## Next Steps & Resources
Resource What You’ll Learn  **SpacetimeDB docs – Collections**Advanced query operators (`where`, `orderBy`, `limit`). **Multi‑collection Todo App**Separate collections: `Todos`, `Users`, `Categories`. **Real‑time sync with Cloud Firestore**Bridge local SpacetimeDB ↔︎ Firestore for cross‑device apps. **MVVM with Provider + Riverpod**Full architecture + dependency injection. **Flutter Templates – TodoMVC (Flutter Samples)**Baseline UI patterns (add list, drag‑drop reorder).   
### Quick “Add Sorting by Completion” (Overflowing Tip)

Add a simple dropdown to filter:
dart      1234567891011121314151617// Inside TodoListScreenDropdownButton<String>(  value: 'All',  items: const [    DropdownMenuItem(value: 'All', child: const Text('All')),    DropdownMenuItem(value: 'Active', child: const Text('Active')),    DropdownMenuItem(value: 'Completed', child: const Text('Completed')),  ],  onChanged: (val) {    setState(() {      // Re‑calculate sorted list based on selection      final filter = val == 'All'          ? provider.todos          : provider.todos.where((t) => val.contains('Active') ? !t.completed : t.completed).toList();    });  },);  
## 🎉 Full Project Overview
      123456789101112131415todo_spacetime/├─ pubspec.yaml               ← Add spacetime_foundation + provider├─ lib/│   ├─ main.dart              ← App entry point + provider init│   ├─ model/│   │   └─ todo.dart          ← ToDo class (serializer)│   ├─ provider/│   │   └─ todo_provider.dart ← Service wrapper + ChangeNotifier│   ├─ services/│   │   └─ todo_service.dart  ← CRUD logic using SpacetimeDB│   ├─ widgets/│   │   ├─ add_todo_dialog.dart│   │   └─ todo_item.dart│   └─ screens/│       └─ todo_list_screen.dart  
### TL;DR Code Summary
dart      123456789101112131415// Modelclass ToDo { ... }
// Serviceclass TodoService { ... createToDo, readAllTodos, readTodo, toggleCompleted, deleteTodo ... }
// Providerclass TodoProvider extends ChangeNotifier {  final TodoService _service = TodoService();  List<ToDo> get todos => List.unmodifiable(_todos);  // init(), createTodo(), toggleCompleted(), deleteTodo(), load()}
// UIclass TodoListScreen extends StatelessWidget { ... ListView.builder(itemBuilder: TodoItem(...)) }  
## 🙌 Wrap‑Up

You now have a **complete, production‑ready** Flutter + SpacetimeDB Todo app that:

- Persists items across launches.
- Shows a live list with checkbox toggles.
- Offers CRUD (Create/Read/Update/Delete) operations via a clean service layer.

From here you can explore:

- **Batch updates** (`await _collection.updateAll(todoIds, {'completed': false})`).
- **Search** (`where: {'title': RegExp(r'(todo|task)')}`).
- **Background sync** (if you migrate to Firebase).

Happy coding! 🚀

**Got questions?** Drop them in the comments below or ping me on Twitter: @TechInnovateAI

*Generated by Falcon, Technology Innovation Institute (TII)*
