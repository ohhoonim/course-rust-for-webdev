
use leptos::prelude::*;

#[derive(Clone, Copy, PartialEq)]
enum Tab {
    Tokens,
    Components,
    Layout,
}

#[component]
pub fn TabBar() -> impl IntoView {
    let active = RwSignal::new(Tab::Tokens);

    view! {
        <section class="panel">
            <h2>"class:active"</h2>
            <div class="tabs">
                <button class:active=move || active.get() == Tab::Tokens on:click=move |_| active.set(Tab::Tokens)>"Tokens"</button>
                <button class:active=move || active.get() == Tab::Components on:click=move |_| active.set(Tab::Components)>"Components"</button>
                <button class:active=move || active.get() == Tab::Layout on:click=move |_| active.set(Tab::Layout)>"Layout"</button>
            </div>
            <p>{move || match active.get() {
                Tab::Tokens => "Design tokens drive a consistent UI.",
                Tab::Components => "Component classes stay small and predictable.",
                Tab::Layout => "Layout classes separate structure from decoration.",
            }}</p>
        </section>
    }
}

