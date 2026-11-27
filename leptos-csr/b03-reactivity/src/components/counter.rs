use leptos::prelude::*;

#[component]
pub fn Counter() -> impl IntoView {
    let (count, set_count) = signal(0_i32);
    let doubled = Memo::new(move |_| count.get() * 2);

    Effect::new(move |_| {
        leptos::logging::log!("count changed to {}", count.get());
    });

    view! {
        <section>
            <h3>"계수기 - signal + memo + effect"</h3>
            <p>"Count: " <strong>{count}</strong></p>
            <p>"Doubled (Memo): " <strong>{doubled}</strong></p>
            <button on:click=move |_| set_count.update(|n| *n += 1)>"+"</button>
            <button  on:click=move |_| set_count.update(|n| *n -= 1)>"-"</button>
            <button  on:click=move |_| set_count.set(0)>"Reset"</button>
            <p>"Open DevTools console to see effect logs."</p>
        </section>
    }
}