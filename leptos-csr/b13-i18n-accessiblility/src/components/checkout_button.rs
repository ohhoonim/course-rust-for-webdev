use leptos::prelude::*;

use crate::models::{t, use_lang};

#[component]
pub fn CheckoutButton() -> impl IntoView {
    let lang = use_lang();
    view! {
        <section class="panel">
            <h2>"Button Label"</h2>
            <button>{move || t(lang.get(), "checkout")}</button>
        </section>
    }
}