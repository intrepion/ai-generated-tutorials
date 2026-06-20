<!--
Source: Z.ai - Advanced AI Chatbot & Agent powered by GLM-5.2
URL: https://chat.z.ai/c/21c4d3a3-fe50-4217-9fba-a4c657cfb52c
Extracted: 2026-06-19
-->

# Building a Real-Time To-Do List with Flutter and SpacetimeDB

Are you tired of building To-Do apps with fake, hard-coded data? It’s time to take your Flutter skills to the next level by integrating a real, reactive database.

In this tutorial, we are going to build a fully functional To-Do List application using **Flutter** for the frontend and **SpacetimeDB** for the backend. SpacetimeDB is a remarkable database that allows you to write your backend logic directly inside the database using WebAssembly, making it incredibly fast and perfect for real-time applications.

We will cover full CRUD functionality: **C**reating, **R**eading, **U**pdating, and **D**eleting tasks, plus toggling a completion status.

## Prerequisites

- **Flutter SDK** installed on your machine.
- A **SpacetimeDB** instance running (you can follow the SpacetimeDB quickstart to get one running locally).
- Basic knowledge of Dart and Flutter.

## Step 1: The SpacetimeDB Backend (Overview)

For this tutorial, we assume you have a SpacetimeDB module running with a `todos` table. SpacetimeDB allows you to interact with it via HTTP endpoints for SQL queries and Reducers (functions) for mutations.

Our backend table looks something like this:

- `id` (Primary Key, Auto-increment)
- `text` (String)
- `completed` (Boolean)

We will use SpacetimeDB's HTTP API to execute SQL queries and call reducers from our Flutter app.

## Step 2: Project Setup

First, let's create a new Flutter project and add the required dependencies. We will use the `http` package to communicate with SpacetimeDB's REST API.

Open your terminal and run:
bash     flutter create flutter_spacetimedb_todocd flutter_spacetimedb_todo  
Add the `http` package:
bash     flutter pub add http  
## Step 3: The Data Model

Let's create a `Todo` model to represent our tasks. Create a file named `todo.dart` in the `lib` folder:
dart     // lib/todo.dartclass Todo {  final int id;  String text;  bool completed;
  Todo({    required this.id,    required this.text,    required this.completed,  });
  factory Todo.fromJson(Map<String, dynamic> json) {    return Todo(      id: json['id'] as int,      text: json['text'] as String,      completed: json['completed'] as bool,    );  }
  Map<String, dynamic> toJson() {    return {      'id': id,      'text': text,      'completed': completed,    };  }}  
## Step 4: The SpacetimeDB Service

Now, we need a service class to handle all interactions with SpacetimeDB. Create `spacetime_service.dart`.

*Note: Replace `YOUR_DB_NAME` and your SpacetimeDB host URL with your actual details.*
dart     // lib/spacetime_service.dartimport 'dart:convert';import 'package:http/http.dart' as http;import 'todo.dart';
class SpacetimeService {  // Update this to your SpacetimeDB HTTP URL  final String baseUrl = 'http://127.0.0.1:3000/database/YOUR_DB_NAME';    // Fetch all To-Dos (Read)  Future<List<Todo>> getTodos() async {    final response = await http.post(      Uri.parse('$baseUrl/sql'),      headers: {'Content-Type': 'application/json'},      body: jsonEncode({"sql": "SELECT * FROM todos ORDER BY id DESC"}),    );
    if (response.statusCode == 200) {      final List<dynamic> data = jsonDecode(response.body);      return data.map((json) => Todo.fromJson(json)).toList();    } else {      throw Exception('Failed to load todos');    }  }
  // Create a new To-Do  Future<void> addTodo(String text) async {    await http.post(      Uri.parse('$baseUrl/call/add_todo'),      headers: {'Content-Type': 'application/json'},      body: jsonEncode({"text": text}),    );  }
  // Update an existing To-Do  Future<void> updateTodo(int id, String newText) async {  
## Step 5: Building the UI

Now for the fun part! We will create a main screen that shows the list, allows creating new items, toggling them, and deleting them. We will also create a detail screen to view and update a specific item.

### 1. Main To-Do List Screen

Replace the contents of `lib/main.dart` with the following:
dart     // lib/main.dartimport 'package:flutter/material.dart';import 'spacetime_service.dart';import 'todo.dart';import 'todo_detail_screen.dart';
void main() {  runApp(const MyApp());}
class MyApp extends StatelessWidget {  const MyApp({super.key});
  @override  Widget build(BuildContext context) {    return MaterialApp(      title: 'SpacetimeDB To-Do',      theme: ThemeData(primarySwatch: Colors.indigo),      home: const TodoListScreen(),    );  }}
class TodoListScreen extends StatefulWidget {  const TodoListScreen({super.key});
  @override  State<TodoListScreen> createState() => _TodoListScreenState();}
class _TodoListScreenState extends State<TodoListScreen> {  final SpacetimeService _service = SpacetimeService();  List<Todo> _todos = [];  bool _isLoading = true;
  @override  
### 2. To-Do Detail Screen (Read Single & Update)

Create a new file called `todo_detail_screen.dart` in the `lib` folder. This screen will display a specific task and allow the user to update its text.
dart     // lib/todo_detail_screen.dartimport 'package:flutter/material.dart';import 'spacetime_service.dart';import 'todo.dart';
class TodoDetailScreen extends StatefulWidget {  final Todo todo;  const TodoDetailScreen({super.key, required this.todo});
  @override  State<TodoDetailScreen> createState() => _TodoDetailScreenState();}
class _TodoDetailScreenState extends State<TodoDetailScreen> {  final SpacetimeService _service = SpacetimeService();  late TextEditingController _textController;
  @override  void initState() {    super.initState();    _textController = TextEditingController(text: widget.todo.text);  }
  // UPDATE  Future<void> _updateTodo() async {    if (_textController.text.isNotEmpty && _textController.text != widget.todo.text) {      await _service.updateTodo(widget.todo.id, _textController.text);      Navigator.pop(context, true); // Pass true to indicate a refresh is needed    } else {      Navigator.pop(context, false);    }  }
  @override  Widget build(BuildContext context) {    return Scaffold(  
## How It All Works Together

- **Showing the List (Read All):** When the `TodoListScreen` initializes, it calls `_fetchTodos()`. This hits the SpacetimeDB SQL endpoint (`SELECT * FROM todos`) and maps the JSON response into a list of `Todo` objects.
- **Creating an Item:** Clicking the Floating Action Button opens a dialog. Entering text and hitting "Add" calls the `add_todo` reducer on SpacetimeDB, inserting a new row. We then refresh the list.
- **Reading an Existing Item:** Tapping a `ListTile` navigates to the `TodoDetailScreen`, passing the specific `Todo` object. The screen displays the ID, current status, and text.
- **Updating an Item:** In the `TodoDetailScreen`, you can change the text in the `TextField` and click the Save icon. This calls the `update_todo` reducer in SpacetimeDB.
- **Toggling Status:** Tapping the Checkbox next to a task calls the `toggle_todo` reducer, flipping the boolean value in the database.
- **Deleting an Item:** Clicking the trash can icon calls the `delete_todo` reducer, removing the record from the database.

## Next Steps: Real-Time Magic

In this tutorial, we used a manual refresh approach (`_fetchTodos()`) after every mutation to keep the UI in sync.

Because we are using **SpacetimeDB**, the true power lies in its real-time capabilities. As a next step, you can upgrade this app to use SpacetimeDB's **WebSocket subscriptions**. By subscribing to the `todos` table, SpacetimeDB will automatically push updates to your Flutter app the moment any data changes, eliminating the need to manually call `_fetchTodos()` entirely!

You now have a fully functioning, database-backed To-Do app in Flutter. Happy coding!
