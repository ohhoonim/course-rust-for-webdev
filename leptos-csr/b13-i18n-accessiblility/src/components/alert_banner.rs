use leptos::prelude::*;

use crate::models::{t, use_lang};

#[component]
pub fn AlertBanner() -> impl IntoView {
    let lang = use_lang();
    let saved = RwSignal::new(false);

    // A new language starts a fresh state, so the old message is never read out again.
    Effect::new(move |_| {
        lang.get();
        saved.set(false);
    });

    view! {
        <section class="panel">
            <h2>"ARIA live region"</h2>
            <button on:click=move |_| saved.set(true)>{move || t(lang.get(), "save")}</button>
            <div class="alert-region" role="status" aria-live="polite">
                {move || saved.get().then(|| view! { <div class="alert">{t(lang.get(), "alert")}</div> })}
            </div>
        </section>
    }
}