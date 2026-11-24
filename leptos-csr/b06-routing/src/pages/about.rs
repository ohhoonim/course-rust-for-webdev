use leptos::prelude::*;

#[component]
pub fn AboutPage() -> impl IntoView {
    view! {
        <section class="panel">
            <h2>"About"</h2>
            <p>"Routes are declared as components and resolved without a page reload."</p>
        </section>
    }
}
