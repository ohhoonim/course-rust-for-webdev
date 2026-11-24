use leptos::prelude::*;

#[component]
pub fn LiveTextInput() -> impl IntoView {
    let text = RwSignal::new(String::new());
    let chars = Memo::new(move |_| text.read().chars().count());

    view! {
        <section class="panel">
            <h2>"Controlled Text Input"</h2>
            <input
                placeholder="Type a preview"
                prop:value=text
                on:input:target=move |event| text.set(event.target().value())
            />
            <p>"Characters: " <strong>{chars}</strong></p>
            <p class="preview">{move || if text.read().is_empty() { "Preview appears here".into() } else { text.get() }}</p>
        </section>
    }
}