use mup::{Markup, markup};

struct Todo {
    id: u32,
    title: &'static str,
    done: bool,
}

fn todo_list(todos: &[Todo]) -> Markup {
    markup! {
        @Markup::fragment("todo-list") {
            ul #("todo-list") {
                @for todo in todos {
                    @let status = if todo.done { "done" } else { "open" };
                    li data-id=todo.id data-status=status {
                        span { @todo.title }
                        button
                            type="button"
                            hx-post=(format!("/todos/{}/toggle", todo.id))
                            hx-target="#todo-list"
                            hx-swap="outerHTML"
                        {
                            "Toggle"
                        }
                    }
                }
            }
        }
    }
}

fn page(todos: &[Todo]) -> Markup {
    let list = todo_list(todos);

    markup! {
        @Markup::doctype()
        html lang="en" {
            body hx-boost="true" {
                header {
                    h1 { "Todos" }
                    p { "The full page and the HTMX fragment come from the same markup." }
                }

                @list
            }
        }
    }
}

fn main() {
    let todos = [
        Todo {
            id: 1,
            title: "Ship mup",
            done: true,
        },
        Todo {
            id: 2,
            title: "Write examples",
            done: false,
        },
    ];
    let page = page(&todos);

    println!("--- full page ---");
    println!("{}", page.as_str());
    println!();
    println!("--- htmx fragment: todo-list ---");
    println!("{}", page.render_fragment("todo-list").as_str());
}
