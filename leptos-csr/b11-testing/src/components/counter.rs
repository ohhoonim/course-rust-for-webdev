use leptos::prelude::*;

#[component]
pub fn Counter() -> impl IntoView {
    let count = RwSignal::new(0_i32);
    view! {
        <div>
            <strong>{count}</strong>
            <button on:click=move |_| count.update(|value| *value += 1)>"+1"</button>
        </div>
    }
}