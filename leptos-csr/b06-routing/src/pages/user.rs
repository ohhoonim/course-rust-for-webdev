use leptos::prelude::*;
use leptos_router::{components::A, hooks::use_params_map};

use crate::model::User;

#[component]
pub fn UserListPage() -> impl IntoView {
    view! {
        <section class="panel">
            <h2>"Users"</h2>
            <ul class="cards">
                {users().into_iter().map(|user| view! {
                    <li>
                        <A href=format!("/users/{}", user.id)>
                            <strong>{user.name}</strong>
                            <span>{format!(" - {}", user.role)}</span>
                        </A>
                    </li>
                }).collect_view()}
            </ul>
        </section>
    }
}

#[component]
pub fn UserDetailPage() -> impl IntoView {
    let params = use_params_map();
    let user_id = move || params.read().get("id").unwrap_or_default();

    view! {
        <section class="panel">
            <h2>"User Detail"</h2>
            <p>"User ID: " <strong>{user_id}</strong></p>
            <A href="/users">"Back to users"</A>
        </section>
    }
}

fn users() -> Vec<User> {
    vec![
        User {
            id: 1,
            name: "Ada Lovelace",
            role: "Compiler pioneer",
        },
        User {
            id: 2,
            name: "Grace Hopper",
            role: "Debugger",
        },
        User {
            id: 3,
            name: "Barbara Liskov",
            role: "Type systems",
        },
        User {
            id: 4,
            name: "Graydon Hoare",
            role: "Rust creator",
        },
        User {
            id: 5,
            name: "Carol Nichols",
            role: "Rust educator",
        },
    ]
}