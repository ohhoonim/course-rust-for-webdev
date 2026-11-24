use leptos::prelude::*;

#[component]
pub fn ThemeToggle() -> impl IntoView {
    let theme = use_context::<RwSignal<Theme>>().expect("theme context");
    view! {
        <button on:click=move |_| {
            theme.update(|value| *value = if *value == Theme::Light { Theme::Dark } else { Theme::Light });
        }>
            {move || if theme.get() == Theme::Light { "Switch to dark" } else { "Switch to light" }}
        </button>
    }
}

#[derive(Clone, Copy, PartialEq)]
pub enum Theme {
    Light,
    Dark,
}

