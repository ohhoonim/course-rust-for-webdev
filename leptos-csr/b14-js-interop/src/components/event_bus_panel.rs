use gloo_events::EventListener;
use leptos::prelude::*;

#[component]
pub fn EventBusPanel() -> impl IntoView {
    let received = RwSignal::new(0_u32);

    // Owned by this component: when the component is removed, the listener is dropped and unregistered.
    let _listener: StoredValue<Option<EventListener>, LocalStorage> =
        StoredValue::new_local(web_sys::window().map(|window| {
            EventListener::new(&window, "lesson-event", move |_| {
                let _ = received.try_update(|count| *count += 1);
            })
        }));

    view! {
        <section class="panel">
            <h2>"CustomEvent bus"</h2>
            <button on:click=move |_| {
                if let Some(window) = web_sys::window() {
                    if let Ok(event) = web_sys::CustomEvent::new("lesson-event") {
                        let _ = window.dispatch_event(&event);
                    }
                }
            }>"Emit event"</button>
            <p role="status">{move || format!("Events received: {}", received.get())}</p>
        </section>
    }
}