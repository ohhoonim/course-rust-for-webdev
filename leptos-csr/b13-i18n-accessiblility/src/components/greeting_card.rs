use leptos::prelude::*;

use crate::models::{t, use_lang};

#[component]
pub fn GreetingCard() -> impl IntoView {
    let lang = use_lang();
    view! {
        <section class="panel">
            <h2>{move || t(lang.get(), "title")}</h2>
            <p>{move || t(lang.get(), "greeting")}</p>
        </section>
    }
}