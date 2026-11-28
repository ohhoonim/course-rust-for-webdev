use leptos::prelude::*;

#[derive(Clone)]
pub struct Todo {
    id: u64,
    title: String,
}

#[component]
pub fn TodoList() -> impl IntoView {
    let draft = RwSignal::new(String::new());
    let todos = RwSignal::new(vec![
        Todo {
            id: 1,
            title: "Render a keyed list".into(),
        },
        Todo {
            id: 2,
            title: "Delete one row without rebuilding the rest".into(),
        },
    ]);

    let next_id = RwSignal::new(3_u64);

    let add_todo = move || {
        let title = draft.read().trim().to_string();
        if title.is_empty() {
            return;
        }
        let id = next_id.get();
        next_id.set(id + 1);
        todos.update(|items| items.push(Todo { id, title }));
        draft.set(String::new());
    };

    view! {
        <section class="panel">
            <h2>"TodoList - keyed <For/>"</h2>
            <form class="row" on:submit=move |event| {
                event.prevent_default();
                add_todo();
            }>
                <input
                    placeholder="New todo"
                    prop:value=draft
                    on:input:target=move |event| draft.set(event.target().value())
                />
                <button type="submit">"Add"</button>
            </form>
            <ul class="list">
                <For
                    each=move || todos.get()
                    key=|todo| todo.id
                    children=move |todo| {
                        let id = todo.id;
                        view! {
                            <li class="row between">
                                <span><code>{format!("#{}", todo.id)}</code> " " {todo.title}</span>
                                <button class="danger" on:click=move |_| {
                                    todos.update(|items| items.retain(|item| item.id != id));
                                }>"Delete"</button>
                            </li>
                        }
                    }
                />
            </ul>
        </section>
    }
}
