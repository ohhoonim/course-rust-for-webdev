use leptos::prelude::*;
use leptos_router::components::A;

#[component]
pub fn NotFoundPage() -> impl IntoView {
    view! {
        <section class="panel error">
            <h2>"404"</h2>
            <p>"No route matched this URL."</p>
            <A href="/">"Return home"</A>
        </section>
    }
}
