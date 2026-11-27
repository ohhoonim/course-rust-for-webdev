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
        <section>
            <h3>"RwSignal + batch()"</h3>
            <p>"Counter: " <strong>{counter}</strong></p>
            <p>"Effect ran " {render_count} " time(s)"</p>
            <button on:click=move |_| counter.update(|n| *n += 1)>"+1"</button>
            <button
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
            <button  on:click=move |_| counter.set(0)>"Reset"</button>
        </section>
    }
}
