use leptos::prelude::*;

#[derive(Clone, Copy, PartialEq)]
pub enum Tab {
    Overview,
    Code,
    Docs,
}

/// Explains the `tab panel` step in this lesson.
///
/// What it does: renders the `TabPanel` component for one focused part of the lesson UI.
/// Why it matters: Keeping this UI section isolated makes the signal, context, action, or routing pattern easier to study.
#[component]
pub fn TabPanel() -> impl IntoView {
    let tab = RwSignal::new(Tab::Overview);
    let button_class = move |current| {
        if tab.get() == current {
            "tab active"
        } else {
            "tab"
        }
    };

    view! {
        <section class="panel">
            <h2>"TabPanel - inline conditionals"</h2>
            <div class="tabs">
                <button class=move || button_class(Tab::Overview) on:click=move |_| tab.set(Tab::Overview)>"Overview"</button>
                <button class=move || button_class(Tab::Code) on:click=move |_| tab.set(Tab::Code)>"Code"</button>
                <button class=move || button_class(Tab::Docs) on:click=move |_| tab.set(Tab::Docs)>"Docs"</button>
            </div>
            {move || match tab.get() {
                Tab::Overview => view! { <p>"Control flow in Leptos composes normal Rust with reactive views."</p> }.into_any(),
                Tab::Code => view! { <pre>"move || if signal.get() { view! { ... } } else { view! { ... } }"</pre> }.into_any(),
                Tab::Docs => view! { <p>"Use <Show/> for simple branches and <For/> for keyed dynamic collections."</p> }.into_any(),
            }}
        </section>
    }
}