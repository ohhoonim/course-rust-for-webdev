use leptos::prelude::*;

#[component]
pub fn Counter() -> impl IntoView {
    let (count, set_count) = signal(0_i32);
    let doubled = Memo::new(move |_| count.get() * 2);

    Effect::new(move |_| {
        leptos::logging::log!("count changed to {}", count.get());
    });

    view! {
        <section style="padding: 1rem; border: 1px solid #e2e8f0; border-radius: 8px; margin: 8px 0;">
            <h3>"Counter - signal + memo + effect"</h3>
            <p>"Count: " <strong>{count}</strong></p>
            <p>"Doubled (Memo): " <strong>{doubled}</strong></p>
            <button style="padding: 6px 12px; margin: 4px;" on:click=move |_| set_count.update(|n| *n += 1)>"+"</button>
            <button style="padding: 6px 12px; margin: 4px;" on:click=move |_| set_count.update(|n| *n -= 1)>"-"</button>
            <button style="padding: 6px 12px; margin: 4px;" on:click=move |_| set_count.set(0)>"Reset"</button>
            <p style="font-size: 0.8rem; color: #6b7280;">"Open DevTools console to see effect logs."</p>
        </section>
    }
}