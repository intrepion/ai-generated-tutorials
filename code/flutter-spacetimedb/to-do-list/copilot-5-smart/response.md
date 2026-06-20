<!--
Source: Microsoft Copilot: Chat
URL: https://copilot.microsoft.com/chats/4WWJSZvQCHkYgQmhDmfwQ
Extracted: 2026-06-19
-->

A To‑Do List app is the perfect “hello world” for learning Flutter on the front end and SpacetimeDB on the backend. It’s small, concrete, and shows off the real magic of SpacetimeDB: state lives in the database, logic runs in the database, and UI updates stream to clients automatically.

Below is a full tutorial‑style blog post you can publish as‑is.

Building a Real‑Time To‑Do List App with Flutter + SpacetimeDB

A To‑Do List is the classic starter project — but when you build it with Flutter and SpacetimeDB, it becomes something much more interesting: a real‑time, multiplayer, reactive application with almost no backend boilerplate.

In this tutorial, you’ll build a complete CRUD workflow:

Show the full list of tasks

Create a new task

Read an existing task

Update a task’s text

Toggle a task’s completion status

Delete a task

All with a backend that has no REST routes, no RPC server, no SQL queries, no WebSocket plumbing.
Just database tables + module functions + subscriptions.

🧠 How SpacetimeDB Changes the Architecture

Traditional architecture:

Code
Copy
Flutter → HTTP/RPC → Server → Database

SpacetimeDB architecture:

Code
Copy
Flutter → SpacetimeDB (data + logic + real‑time sync)

Your Rust code becomes a database module.
Your Flutter app subscribes to tables and receives updates instantly.

🗂️ Step 1 — Define the To‑Do Table (Rust)

Inside your SpacetimeDB module (lib.rs):

Rust
Copy
use spacetime::prelude::*;

#[table]
pub struct Todo {
    #[primary_key]
    pub id: u64,
    pub title: String,
    pub completed: bool,
}

This creates a table with:

id — primary key

title — the task text

completed — boolean toggle

SpacetimeDB automatically generates:

storage

indexing

subscriptions

change propagation

⚙️ Step 2 — Write Module Functions (CRUD)

These functions run inside the database engine.

Create
Rust
Copy
#[spacetime_fn]
pub fn create_todo(title: String) {
    Todo::insert(Todo {
        id: Todo::next_id(),
        title,
        completed: false,
    });
}

Read (implicit via subscription)

No function needed — clients subscribe to the table.

Update
Rust
Copy
#[spacetime_fn]
pub fn update_todo(id: u64, new_title: String) {
    if let Some(mut todo) = Todo::get(id) {
        todo.title = new_title;
        todo.update();
    }
}

Toggle Complete
Rust
Copy
#[spacetime_fn]
pub fn toggle_complete(id: u64) {
    if let Some(mut todo) = Todo::get(id) {
        todo.completed = !todo.completed;
        todo.update();
    }
}

Delete
Rust
Copy
#[spacetime_fn]
pub fn delete_todo(id: u64) {
    Todo::delete(id);
}

That’s the entire backend.

📡 Step 3 — Connect Flutter to SpacetimeDB

Add the SpacetimeDB Flutter client:

Yaml
Copy
dependencies:
  spacetime_client: ^0.1.0

Initialize the client:

Dart
Copy
final client = SpacetimeClient("wss://your-db-url");
await client.connect();

Subscribe to the Todo table:

Dart
Copy
final todos = client.table<Todo>("Todo").subscribe();

This gives you a real‑time stream of rows.

🖥️ Step 4 — Show the To‑Do List
Dart
Copy
StreamBuilder<List<Todo>>(
  stream: todos.stream,
  builder: (context, snapshot) {
    final items = snapshot.data ?? [];

    return ListView(
      children: [
        for (final todo in items)
          ListTile(
            title: Text(todo.title),
            leading: Checkbox(
              value: todo.completed,
              onChanged: (_) => client.call("toggle_complete", [todo.id]),
            ),
            trailing: IconButton(
              icon: Icon(Icons.delete),
              onPressed: () => client.call("delete_todo", [todo.id]),
            ),
            onTap: () {
              // navigate to detail page
            },
          )
      ],
    );
  },
)

Every time the database changes, the UI updates automatically.

➕ Step 5 — Create a New To‑Do
Dart
Copy
void createTodo() {
  client.call("create_todo", [controller.text]);
  controller.clear();
}

Attach to a button:

Dart
Copy
FloatingActionButton(
  onPressed: createTodo,
  child: Icon(Icons.add),
)

🔍 Step 6 — Read a Single To‑Do Item

You can read from the subscription:

Dart
Copy
final todo = todos.stream.value.firstWhere((t) => t.id == id);

Or subscribe to a filtered view:

Dart
Copy
final singleTodo = client
  .table<Todo>("Todo")
  .where((t) => t.id.equals(id))
  .subscribe();

✏️ Step 7 — Update a To‑Do Item
Dart
Copy
void updateTodoTitle(int id, String newTitle) {
  client.call("update_todo", [id, newTitle]);
}

Use it in a text field:

Dart
Copy
TextField(
  controller: controller,
  onSubmitted: (value) => updateTodoTitle(todo.id, value),
)

✔️ Step 8 — Toggle Completion

Already shown above, but here’s the standalone call:

Dart
Copy
client.call("toggle_complete", [todo.id]);

🗑️ Step 9 — Delete a To‑Do Item
Dart
Copy
client.call("delete_todo", [todo.id]);

🎉 Final Thoughts

You now have a fully functional, real‑time To‑Do List app powered by:

Flutter for UI

SpacetimeDB for backend logic, storage, and real‑time sync

What you didn’t need:

REST endpoints

RPC server

SQL queries

WebSockets

State management boilerplate

SpacetimeDB collapses the backend into a single concept:
your logic runs where your data lives.

If you want, I can generate:

A full GitHub‑ready project structure

A more advanced version (tags, due dates, user accounts)

A multiplayer collaborative mode

Which direction do you want to take this next?

Edit in a page
Message Copilot
Smart
