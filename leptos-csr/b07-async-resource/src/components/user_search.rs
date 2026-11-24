use leptos::prelude::*;

use crate::models::fetch_users;

#[component]
pub fn UserSearch() -> impl IntoView {
    let query = RwSignal::new(String::new());
    let users = LocalResource::new(move || fetch_users(query.get()));

    view! {
        <section class="panel">
            <h2>"UserSearch - Transition keeps old results"</h2>
            <input
                placeholder="Search by name or email"
                prop:value=query
                on:input:target=move |event| query.set(event.target().value())
            />
            <Transition fallback=|| view! { <p>"Searching..."</p> }>
                {move || users.get().map(|result| match result {
                    Ok(users) => view! {
                        <ul class="cards">
                            {users.into_iter().map(|user| view! {
                                <li>
                                    <strong>{format!("{}: {}", user.id, user.name)}</strong>
                                    <span>{format!(" - {}", user.email)}</span>
                                </li>
                            }).collect_view()}
                        </ul>
                    }.into_any(),
                    Err(error) => view! { <p class="error">{error}</p> }.into_any(),
                })}
            </Transition>
        </section>
    }
}