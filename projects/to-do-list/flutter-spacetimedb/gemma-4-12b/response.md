<!--
Source: write me a tutorial blog post on a To Do List project using Flutter and SpacetimeDB. It should have the basics of showing the whole list, creating a new To Do item, reading an existing To Do item, updating an existing To Do item, some sort of complete status toggle, and deleting an existing To Do item. - llama-ui
URL: http://localhost:8080/#/chat/5f927fab-6b09-447b-9641-d02d8bc78657
Extracted: 2026-06-19
-->

This is a technical tutorial designed for a blog post. It covers the integration of **Flutter** (for the UI) and **SpacetimeDB** (for the real-time, distributed database backend).

# Building a Real-Time To-Do List with Flutter and SpacetimeDB

In the modern era of app development, "real-time" is no longer a luxury—it’s an expectation. Whether it’s a chat app or a collaborative task manager, users expect data to sync instantly across devices.

Today, we are going to build a **To-Do List App** using **Flutter** and **SpacetimeDB**.

### Why SpacetimeDB?

Unlike traditional databases where you have to write complex APIs to sync data, SpacetimeDB allows you to write your logic in a "Smart Contract" style. When the data changes in the database, your Flutter app updates automatically. No WebSockets or manual polling required.

## Prerequisites

- Flutter SDK installed.
- SpacetimeDB installed and running locally.
- Basic knowledge of Dart and Flutter.

dart
```
class TodoScreen extends StatefulWidget {
  @override
  _TodoScreenState createState() => _TodoScreenState();
}

class _TodoScreenState extends State<TodoScreen> {
  final TodoService _todoService = TodoService();
  List<TodoItem> _todoList = [];
  final TextEditingController _controller = TextEditingController();

  @override
  void initState() {
    super.initState();
    _refreshList();
  }

  void _refreshList() async {
    var data = await _todoService.fetchAll();
    setState(() {
      _todoList = data;
    });
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(title: Text("Spacetime To-Do")),
      body: Column(
        children: [
          Padding(
            padding: EdgeInsets.all(16.0),
            child: TextField(
              controller: _controller,
              decoration: InputDecoration(
                hintText: "Enter task...",
                suffixIcon: IconButton(
                  icon: Icon(Icons.add),
                  onPressed: () async {
                    await _todoService.addTodo(_controller.text);
                    _controller.clear();
                    _refreshList();
                  },
                ),
              ),
            ),
          ),
          Expanded(
            child: ListView.builder(
              itemCount: _todoList.length,
              itemBuilder: (context, index) {
                final item = _todoList[index];
                return ListTile(
                  title: Text(item.title),
                  trailing: Checkbox(
                    value: item.isCompleted,
                    onChanged: (val) async {
                      await _todoService.toggleStatus(item.id);
                      _refreshList();
                    },
                  ),
                  leading: IconButton(
                    icon: Icon(Icons.delete, color: Colors.red),
                    onPressed: () async {
                      await _todoService.deleteTodo(item.id);
                      _refreshList();
                    },
                  ),
                );
              },
            ),
          ),
        ],
      ),
    );
  }
}
```

## How it all works together:

- **Create:** When you type a task and hit "Add", the `addTodo` function calls the SpacetimeDB `create_todo` action. The database generates a unique ID and stores it.
- **Read:** The `fetchAll` function pulls the entire list. Because SpacetimeDB is reactive, you can even set up a listener so the UI updates automatically when *other* users add tasks.
- **Update:** The `toggleStatus` function targets a specific ID and flips the boolean.
- **Delete:** The `deleteTodo` function removes the record from the ledger entirely.

## Conclusion

By combining Flutter's beautiful UI components with SpacetimeDB’s "Logic-in-the-Database" approach, we eliminated the need for a complex backend API. We have a real-time, scalable To-Do app in minutes!
 
**Next Steps:** Try adding a "Priority" level to your items or a "Due Date" field to see how SpacetimeDB handles more complex types!
