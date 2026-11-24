use leptos::prelude::*;

use crate::models::{t, use_lang};

#[component]
pub fn AccessibleForm() -> impl IntoView {
    let lang = use_lang();
    let name = RwSignal::new(String::new());
    view! {
        <section class="panel">
            <h2>"Accessible Form"</h2>
            <form class="stack" on:submit=move |event| event.prevent_default()>
                <label for="name-input">{move || t(lang.get(), "name")}</label>
                <input
                    id="name-input"
                    autocomplete="name"
                    prop:value=name
                    on:input:target=move |event| name.set(event.target().value())
                />
                <button type="submit">{move || t(lang.get(), "submit")}</button>
            </form>
        </section>
    }
}