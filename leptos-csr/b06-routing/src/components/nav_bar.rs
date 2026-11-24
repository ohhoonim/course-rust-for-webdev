use leptos::prelude::*;
use leptos_router::components::A;

#[component]
pub fn NavBar() -> impl IntoView {
    view! {
        <nav>
            <A href="/" exact=true>"Home"</A>
            <A href="/about">"About"</A>
            <A href="/users">"Users"</A>
            <A href="/dashboard">"Dashboard"</A>
            <A href="/dashboard/settings">"Settings"</A>
            <A href="/contact">"Contact"</A>
        </nav>
    }
}
