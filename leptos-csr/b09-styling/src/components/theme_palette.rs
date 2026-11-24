use leptos::prelude::*;

#[component]
pub fn ThemePalette() -> impl IntoView {
    let themes = [
        ("Ocean", "#2563eb"),
        ("Forest", "#047857"),
        ("Rose", "#be123c"),
        ("Amber", "#b45309"),
        ("Graphite", "#374151"),
    ];
    let active = RwSignal::new("#2563eb".to_string());

    Effect::new(move |_| {
        if let Some(root) = web_sys::window()
            .and_then(|window| window.document())
            .and_then(|document| document.document_element())
        {
            let _ = root.set_attribute("style", &format!("--primary: {}", active.get()));
        }
    });

    view! {
        <section class="panel">
            <h2>"CSS custom properties"</h2>
            <div class="swatches">
                {themes.into_iter().map(|(name, color)| view! {
                    <button
                        class:active=move || active.get() == color
                        style:background=color
                        on:click=move |_| active.set(color.to_string())
                        title=name
                    >
                        {name}
                    </button>
                }).collect_view()}
            </div>
            <p class="brand">"This text uses var(--primary)."</p>
        </section>
    }
}
