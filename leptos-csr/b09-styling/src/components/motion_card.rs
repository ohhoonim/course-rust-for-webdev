use leptos::prelude::*;

#[component]
pub fn MotionCard() -> impl IntoView {
    let visible = RwSignal::new(true);
    view! {
        <section class="panel">
            <h2>"Reactive style attributes"</h2>
            <button on:click=move |_| visible.update(|value| *value = !*value)>"Toggle card"</button>
            <div
                class="motion-card"
                style:opacity=move || if visible.get() { "1" } else { "0.35" }
                style:transform=move || if visible.get() { "translateY(0)" } else { "translateY(16px)" }
                style:color=move || if visible.get() { "var(--primary)" } else { "#64748b" }
            >
                "Opacity, transform, and color are all driven by signals."
            </div>
        </section>
    }
}

