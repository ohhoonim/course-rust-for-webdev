use leptos::prelude::*;
use leptos_router::hooks::use_navigate;

#[component]
pub fn HomePage() -> impl IntoView {
    let navigate = use_navigate();
    view! {
        <section class="panel">
            <h2>"Home"</h2>
            <p>"This page demonstrates client-side navigation with the Leptos 0.8 router."</p>
            <button on:click=move |_| navigate("/dashboard", Default::default())>
                "Go to Dashboard"
            </button>
        </section>
    }
}
