use leptos::{html, prelude::*};

#[component]
pub fn SearchNodeRef() -> impl IntoView {
    let search_ref: NodeRef<html::Input> = NodeRef::new();
    let submitted = RwSignal::new(String::from("Nothing searched yet"));

    view! {
        <section class="panel">
            <h2>"Uncontrolled Search with NodeRef"</h2>
            <form class="row" on:submit=move |event| {
                event.prevent_default();
                if let Some(input) = search_ref.get() {
                    submitted.set(input.value());
                }
            }>
                <input node_ref=search_ref placeholder="Read only on submit"/>
                <button type="submit">"Search"</button>
            </form>
            <p>{submitted}</p>
        </section>
    }
}