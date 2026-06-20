<!--
Source: Qwen Studio
URL: https://chat.qwen.ai/c/6b137fdc-99a2-4eaa-afc7-546ca97b78db
Extracted: 2026-06-19
-->

# The Death of the Refetch: Building a Reactive To-Do App with Flutter & SpacetimeDB
Most Flutter tutorials teach you to treat your backend like a vending machine: insert API call, wait, get data, update UI. SpacetimeDB breaks this paradigm. It doesn't just store data; it streams it. In this tutorial, we will build a To-Do app where **you never manually update your UI state after a mutation**. The database pushes the changes.
## 1. The Backend: SpacetimeDB (Rust)
First, we define our module. Notice we aren't writing REST endpoints. We are writing **reducers** (mutations) and defining a **table** (state).
```
rust
1
2
3
4
5
6
7
8
9
10
11
12
13
14
15
16
17
18
19
20
21
22
23
24
25
26
27
28
29
30
31
32
33
34
35
36
37
38
39
40
41
42
43
44
// module.rs
use spacetimedb::{spacetimedb, Table, ReducerContext};
#[spacetimedb(table(public))]
pub struct Todo {
    #[primarykey]
    pub id: u64,
    pub title: String,
    pub is_completed: bool,
}
// CREATE
#[spacetimedb(reducer)]
pub fn create_todo(ctx: &ReducerContext, title: String) {
    ctx.db.todo().insert(Todo {
        id: ctx.rng().gen(),
        title,
        is_completed: false,
    });
}
// UPDATE & TOGGLE
#[spacetimedb(reducer)]
pub fn toggle_todo(ctx: &ReducerContext, id: u64) {
    if let Some(mut todo) = ctx.db.todo().id().find(id) {
        todo.is_completed = !todo.is_completed;
        ctx.db.todo().id().update(todo);
    }
}
// UPDATE (Title)
#[spacetimedb(reducer)]
pub fn update_todo_title(ctx: &ReducerContext, id: u64, new_title: String) {
    if let Some(mut todo) = ctx.db.todo().id().find(id) {
        todo.title = new_title;
        ctx.db.todo().id().update(todo);
    }
}
// DELETE
#[spacetimedb(reducer)]
pub fn delete_todo(ctx: &ReducerContext, id: u64) {
    ctx.db.todo().id().delete(id);
}
```

## 2. The Frontend: Flutter (Dart)
Here is where we diverge from standard Flutter practices. We will use SpacetimeDB's reactive subscriptions. 
### The Setup & Connection
Initialize the client and subscribe to the `Todo` table. This subscription is a live stream.
```
dart
1
2
3
4
5
6
7
8
9
10
11
12
13
import 'package:spacedb/spacedb.dart';
class TodoService {
  static final SpacetimeDBClient client = SpacetimeDBClient.instance;
  static Future<void> connect() async {
    await client.connect('ws://localhost:3000', 'todo_db');
    
    // THE BUTTER: Subscribe to the table. 
    // This replaces your Riverpod/Bloc data fetching logic.
    client.subscribe('SELECT * FROM Todo;');
  }
}
```

### The Reactive UI
Instead of using a `FutureBuilder` or managing a `List<Todo>` in a `StatefulWidget`, we listen to the SpacetimeDB stream.
```
dart
1
2
3
4
5
6
7
8
9
10
11
12
13
14
15
16
17
18
19
20
21
22
23
24
25
26
27
28
29
30
31
32
33
34
35
36
37
38
39
40
41
42
43
44
45
46
47
48
49
50
51
52
53
54
55
56
57
58
59
60
61
62
63
64
65
66
67
68
69
70
class TodoListScreen extends StatefulWidget {
  @override
  _TodoListScreenState createState() => _TodoListScreenState();
}
class _TodoListScreenState extends State<TodoListScreen> {
  List<Todo> _todos = [];
  late StreamSubscription _subscription;
  @override
  void initState() {
    super.initState();
    
    // Listen to the reactive stream
    _subscription = SpacetimeDBClient.instance.onTableUpdate('Todo').listen((rows) {
      setState(() {
        _todos = rows.map((r) => Todo.fromSpacetime(r)).toList();
      });
    });
  }
  @override
  void dispose() {
    _subscription.cancel();
    super.dispose();
  }
  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(title: Text('Reactive To-Do')),
      body: ListView.builder(
        itemCount: _todos.length,
        itemBuilder: (context, index) {
          final todo = _todos[index];
          return ListTile(
            title: Text(todo.title),
            leading: Checkbox(
              value: todo.isCompleted,
              // TOGGLE: Fire and forget. The DB will push the update back.
              onChanged: (_) => _toggleTodo(todo.id), 
            ),
            trailing: IconButton(
              icon: Icon(Icons.delete),
              // DELETE: Fire and forget.
              onPressed: () => _deleteTodo(todo.id), 
            ),
          );
        },
      ),
      floatingActionButton: FloatingActionButton(
        // CREATE: Fire and forget.
        onPressed: () => _createTodo('New Task ${DateTime.now().second}'), 
      ),
    );
  }
  // --- Mutations (Reducers) ---
  void _createTodo(String title) {
    SpacetimeDBClient.instance.callReducer('create_todo', [title]);
  }
  void _toggleTodo(int id) {
    SpacetimeDBClient.instance.callReducer('toggle_todo', [id]);
  }
  void _deleteTodo(int id) {
    SpacetimeDBClient.instance.callReducer('delete_todo', [id]);
  }
}
```
*(Note: "Reading an existing item" is inherently handled by the `SELECT *` subscription, which streams all rows. If you need a single item, you subscribe to `SELECT * FROM Todo WHERE id = ?`)*
## 🦉 Sparring Partner: Your Blind Spots
As your sparring partner, I need to point out where developers typically fail when transitioning to this architecture:
- **The "Optimistic UI" Trap:** In REST/GraphQL, you update the local UI state *immediately* when a user clicks "delete", then call the API, and roll back if it fails. **Don't do this here.** Because SpacetimeDB pushes the state change back via the subscription in milliseconds, manually updating local state creates race conditions and duplicate UI flickers. Let the database be the single source of truth.
- **Over-engineering State Management:** You do not need complex Bloc or Riverpod setups for the data layer of this app. The SpacetimeDB stream *is* your state manager. Use Flutter's native state management only for ephemeral UI state (like text field inputs or modal visibility).
- **The Identity Blind Spot:** This tutorial uses a public table. In the real world, a To-Do app requires user isolation. Your next step must be implementing SpacetimeDB's `Identity` system to filter the subscription: `SELECT * FROM Todo WHERE owner_id = ?`.

**Confidence Level:** 
I am **95% confident** in the architectural patterns and the Rust/SpacetimeDB reducer logic. I am **90% confident** in the exact Flutter SDK method names (e.g., `onTableUpdate`), as the Dart SDK is actively evolving, but the reactive stream paradigm remains exactly as described. Let me know if you want to drill deeper into the identity/auth layer or row-level security.AutoAI-generated content may not be accurate.
