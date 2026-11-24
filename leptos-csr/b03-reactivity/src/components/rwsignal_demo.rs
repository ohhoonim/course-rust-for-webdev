use leptos::prelude::*;

#[component]
pub fn RwSignalDemo() -> impl IntoView {
    let counter = RwSignal::new(0_i32);
    let (render_count, set_render_count) = signal(0_u32);

    Effect::new(move |_| {
        let _ = counter.get();
        set_render_count.update(|n| *n += 1);
    });

    view! {
        <section style="padding: 1rem; border: 1px solid #e2e8f0; border-radius: 8px; margin: 8px 0;">
            <h3>"RwSignal + batch()"</h3>
            <p>"Counter: " <strong>{counter}</strong></p>
            <p style="font-size: 0.85rem; color: #6b7280;">"Effect ran " {render_count} " time(s)"</p>
            <button style="padding: 6px 12px; margin: 4px;" on:click=move |_| counter.update(|n| *n += 1)>"+1"</button>
            <button
                style="padding: 6px 12px; margin: 4px;"
                on:click=move |_| {
                    batch(move || {
                        counter.update(|n| *n += 10);
                        counter.update(|n| *n += 10);
                        counter.update(|n| *n += 10);
                    });
                }
            >
                "+30 (batched)"
            </button>
            <button style="padding: 6px 12px; margin: 4px;" on:click=move |_| counter.set(0)>"Reset"</button>
        </section>
    }
}