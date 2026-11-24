use leptos::prelude::*;

use crate::models::{t, use_lang};

#[component]
pub fn DirectionPreview() -> impl IntoView {
    let lang = use_lang();
    view! {
        <section class="panel">
            <h2>{move || t(lang.get(), "direction")}</h2>
            <div class="direction-card" dir=move || lang.get().dir()>
                {move || t(lang.get(), "direction_note")}
            </div>
        </section>
    }
}