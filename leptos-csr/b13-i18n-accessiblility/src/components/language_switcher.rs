use leptos::prelude::*;

use gloo_storage::{LocalStorage, Storage};

use crate::models::{Lang, STORAGE_KEY, use_lang};


#[component]
pub fn LanguageSwitcher() -> impl IntoView {
    let lang = use_lang();
    view! {
        <section class="panel">
            <h2>"Language Switcher"</h2>
            <div class="row" role="group" aria-label="Language">
                {   Lang::ALL
                    .into_iter()
                    .map(|option| {
                        view! {
                            <button
                                lang=option.code()
                                aria-pressed=move || if lang.get() == option { "true" } else { "false" }
                                class:active=move || lang.get() == option
                                on:click=move |_| choose(lang, option)
                            >
                                {option.native_name()}
                            </button>
                        }
                    })
                    .collect_view()}
            </div>
        </section>
    }
}


fn choose(lang: RwSignal<Lang>, option: Lang) {
    lang.set(option);
    let _ = LocalStorage::set(STORAGE_KEY, option.code());
}