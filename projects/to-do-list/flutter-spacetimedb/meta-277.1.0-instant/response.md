<!--
Source: Meta AI
URL: https://www.meta.ai/prompt/f59fc073-adee-4147-9dec-617f576a54b1
Extracted: 2026-06-19
-->

Here’s a complete tutorial blog post you can drop into your dev blog. We’ll build a realtime To Do List with Flutter + SpacetimeDB that does full CRUD + a “complete” toggle.

SpacetimeDB replaces your backend: you write Rust “reducers” instead of API endpoints, and Flutter gets a live, synced view of every table. The Dart SDK makes tables feel like `ValueNotifier`s and reducers feel like typed function calls.

### What we’ll build

- Create: Add new todos
- Read: Show the whole list, live-updating across clients
- Update: Edit todo text
- Complete: Toggle done/undone status
- Delete: Remove todos

### 1. Setup SpacetimeDB + Flutter

1.1 Install SpacetimeDB CLI
Bash
1.3 Define the table + reducers `src/lib.rs`:
Rust
The pattern: define a `#[table]` struct, then `#[reducer]` functions that insert/update/delete.

1.4 Publish locally
Bash
`client.connect` takes `initialSubscriptions` and your table becomes a live list.

### 3. CRUD UI in Flutter

3.1 Read: Show the whole list

Every table is a `ValueNotifier`. Iterate `client.todo.iter()` or listen to changes.
dart
Key CRUD calls:

Action

Code

Create

`client.reducers.addTodo(text)`

Read

`client.todo.rows.value` + `ValueListenableBuilder`

Update

`client.reducers.updateTodo(id, newText)`

Toggle Complete

`client.reducers.toggleComplete(id)`

Delete

`client.reducers.deleteTodo(id)`

Reducers are typed function calls from the generated client.

### 4. How it works

- Tables define your data shape. `#[table(public)]` makes it readable by all clients.
- Reducers are your API. They run transactionally on the server.
- Subscriptions `SELECT * FROM todo` give you live data. No polling.
- Optimistic updates: The SDK applies your reducer’s writes locally before server confirm, so UI feels instant.

### 5. Next steps

- Deploy: `spacetime login` then `spacetime publish todo-db` to Maincloud.
- Auth: SpacetimeDB knows who called via `ctx.sender`. Add login UI.
- Offline: Enable offline storage + optimistic writes for offline-first.
- Row-level security: Add `#[spacetimedb::filter]` to lock todos to owner.

You now have a realtime, multi-client todo app with zero backend code beyond Rust reducers. Every client sees updates instantly because SpacetimeDB pushes changes to subscribed clients automatically.

Want me to add authentication with OpenAuth or expand this with due dates + tags?
Ask Meta AI...
