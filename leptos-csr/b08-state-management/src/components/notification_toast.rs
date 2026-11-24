use leptos::prelude::*;

use crate::models::{Level, Notification};

#[component]
pub fn NotificationToast() -> impl IntoView {
    let notifications = use_context::<RwSignal<Vec<Notification>>>().expect("notification context");

    view! {
        <div class="toasts">
            <For
                each=move || notifications.get()
                key=|notification| notification.id
                children=move |notification| {
                    let class = match notification.level {
                        Level::Info => "toast info",
                        Level::Success => "toast success",
                        Level::Warning => "toast warning",
                    };
                    view! { <div class=class>{notification.message}</div> }
                }
            />
        </div>
    }
}

