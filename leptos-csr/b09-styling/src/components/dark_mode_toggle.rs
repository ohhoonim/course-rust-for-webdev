use leptos::prelude::*;

#[component]
pub fn DarkModeToggle() -> impl IntoView {
    let dark = RwSignal::new(false);

    Effect::new(move |_| {
        if let Some(root) = web_sys::window()
            .and_then(|window| window.document())
            .and_then(|document| document.document_element())
        {
            let class_name = if dark.get() { "dark" } else { "" };
            let _ = root.set_attribute("class", class_name);
        }
    });

    view! {
        <section class="panel">
            <h2>"Dark mode root class"</h2>
            <button on:click=move |_| dark.update(|value| *value = !*value)>
                {move || if dark.get() { "Disable dark mode" } else { "Enable dark mode" }}
            </button>
        </section>
    }
}