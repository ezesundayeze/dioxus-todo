use dioxus::{html::input_data::keyboard_types::Key, prelude::*};

/// Define a type for Todos, which is an immutable HashMap of u32 keys to TodoItem values.
pub type Todos = im::HashMap<u32, TodoItem>;

/// Define the structure for a TodoItem, which has an id and contents.
#[derive(Debug, PartialEq, Clone)]
pub struct TodoItem {
    pub id: u32,
    pub contents: String,
}

/// Define the app function that returns an Element.
/// This function initializes the main application and manages todos, new todo input, and todo IDs.
pub fn app(cx: Scope<()>) -> Element {
    // Create references for todos, new_todo_item, and todo_id.
    let todos: &UseRef<im::HashMap<u32, TodoItem>> =
        use_ref(cx, || {
            let mut default_todos = im::HashMap::<u32, TodoItem>::default();
            // Add a default todo item.
            // default_todos.insert(0, TodoItem { id: 0, contents: "Cut Onion".to_string() });
            default_todos
        });
    let new_todo_item: &UseRef<String> = use_ref(cx, String::new);
    let todo_id: &UseState<u32> = use_state(cx, || 0); 

    // Render the app using rsx! macro.
    cx.render(rsx! {
        section { class: "todo-app",
            style { include_str!("./style.css") }
            div {
                header { class: "header",
                    h1 {"Todo App"}
                    todo_input { todos: todos.clone(), new_todo_item: new_todo_item, todo_id: todo_id }
                }
                todo_list { todos: todos.clone() }
            }
        }
    })
}


/// Define the properties structure for TodoInput.
#[derive(Props, PartialEq)]
pub struct TodoInputProps<'a> {
    todos: UseRef<im::HashMap<u32, TodoItem>>,
    new_todo_item: &'a UseRef<String>,
    todo_id: &'a UseState<u32>,
}

/// Define the todo_input function that returns an Element.
/// This function renders the input field for adding new todos and handles user input.
pub fn todo_input<'a>(cx: Scope<'a, TodoInputProps>) -> Element<'a> {
    
    // Render the input element with properties.
    cx.render(rsx! {
        input {
            class: "new-todo",
            placeholder: "Add Todo",
            value: "{cx.props.new_todo_item.read()}",
            autofocus: true,
            oninput: move |event| cx.props.new_todo_item.set(event.value.clone()),
            onkeydown: move |event| {
                if event.key() == Key::Enter && !cx.props.new_todo_item.read().is_empty() {
                    cx.props.todos.write().insert(
                        *cx.props.todo_id.get(),
                        TodoItem {
                            id: *cx.props.todo_id.get(),
                            contents: cx.props.new_todo_item.read().clone(),
                        },
                    );
                    cx.props.todo_id.set(cx.props.todo_id + 1);
                    cx.props.new_todo_item.set("".to_string());
                }
            }
        }
    })
}

/// Define the properties structure for TodoList.
#[derive(Props, PartialEq)]
pub struct TodoListProps {
    todos: UseRef<im::HashMap<u32, TodoItem>>,
}

/// Define the todo_list function that returns an Element.
/// This function renders the list of todos.
pub fn todo_list<'a>(cx: Scope<'a, TodoListProps>) -> Element {
    // Render an unordered list with todo entries.
    cx.render(rsx! {ul { class: "todo-list",
    cx.props.todos.read().iter().map(|(id, _todo)| {
        rsx! { todo_entry { key: "{id}", id: *id, set_todos: &cx.props.todos } }
    })
    }
    })
}

/// Define the properties structure for TodoEntry.
#[derive(Props)]
pub struct TodoEntryProps<'a> {
    set_todos: &'a UseRef<Todos>,
    id: u32,
}

/// Define the todo_entry function that returns an Element.
/// This function renders a single todo entry.
pub fn todo_entry<'a>(cx: Scope<'a, TodoEntryProps<'a>>) -> Element {
    // Retrieve the todos and the current todo using the provided id.
    let todos: std::cell::Ref<'_, im::HashMap<u32, TodoItem>> = cx.props.set_todos.read();
    let todo: &TodoItem = &todos[&cx.props.id];

    // Render a list item with the todo's contents and a delete button.
    render!(li {
        div { class: "view",
            label { "{todo.contents}" }
            button {
                class: "remove",
                onclick: move |_| {
                    cx.props.set_todos.write().remove(&cx.props.id);
                }
            }
        }
    })
}
