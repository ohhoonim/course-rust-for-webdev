use leptos::prelude::*;

use gloo_storage::{LocalStorage, Storage};

#[component]
pub fn PersistentCounter() -> impl IntoView {
    let count = RwSignal::new(LocalStorage::get::<i32>("counter_v1").unwrap_or(0));

    Effect::new(move |_| {
        let _ = LocalStorage::set("counter_v1", count.get());
    });

    view! {
        <section class="panel">
            <h2>"LocalStorage Persistence"</h2>
            <p>"Counter: " <strong>{count}</strong></p>
            <button on:click=move |_| count.update(|value| *value += 1)>"+1"</button>
            <button on:click=move |_| count.set(0)>"Reset"</button>
        </section>
    }
}
