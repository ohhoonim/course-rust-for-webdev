use leptos::prelude::*;

use crate::models::{Level, use_notify};

#[component]
pub fn NotificationDemo() -> impl IntoView {
    let notify = use_notify();
    let notify_info = notify.clone();
    let notify_success = notify.clone();
    view! {
        <section class="panel">
            <h2>"Notification System"</h2>
            <button on:click=move |_| notify_info("Info notification".into(), Level::Info)>"Info"</button>
            <button on:click=move |_| notify_success("Saved successfully".into(), Level::Success)>"Success"</button>
            <button on:click=move |_| notify("Check your input".into(), Level::Warning)>"Warning"</button>
        </section>
    }
}
