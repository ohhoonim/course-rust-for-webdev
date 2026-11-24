use leptos::prelude::*;

#[component]
pub fn BatchingBenchmark() -> impl IntoView {
    let a = RwSignal::new(0_i32);
    let b = RwSignal::new(0_i32);
    let c = RwSignal::new(0_i32);
    let unbatched = RwSignal::new(0.0_f64);
    let batched = RwSignal::new(0.0_f64);
    // Counts how often an effect that reads all three signals runs. This is the honest measure:
    // the bars below are smaller than the browser's timer precision, so treat them as noise.
    let effect_runs = RwSignal::new(0_u32);
    Effect::new(move |_| {
        let _ = (a.get(), b.get(), c.get());
        effect_runs.update(|runs| *runs += 1);
    });

    view! {
        <section class="panel">
            <h2>"Signal batching benchmark"</h2>
            <p>{move || format!("a={} b={} c={}", a.get(), b.get(), c.get())}</p>
            <p>{move || format!("effect runs: {}", effect_runs.get())}</p>
            <p>"Press either button: the effect runs once for each, because Leptos runs effects after your handler finishes. The timing bars are below the browser's timer precision, so read them as noise."</p>
            <button on:click=move |_| {
                let start = now();
                a.update(|value| *value += 1);
                b.update(|value| *value += 1);
                c.update(|value| *value += 1);
                unbatched.set((now() - start).max(0.1));
            }>"Unbatched update"</button>
            <button on:click=move |_| {
                let start = now();
                batch(move || {
                    a.update(|value| *value += 1);
                    b.update(|value| *value += 1);
                    c.update(|value| *value += 1);
                });
                batched.set((now() - start).max(0.1));
            }>"Batched update"</button>
            <Bar label="unbatched" value=unbatched/>
            <Bar label="batched" value=batched/>
        </section>
    }
}


fn now() -> f64 {
    web_sys::window()
        .and_then(|window| window.performance())
        .map(|performance| performance.now())
        .unwrap_or_default()
}

#[component]
fn Bar(label: &'static str, value: RwSignal<f64>) -> impl IntoView {
    view! {
        <div class="bar-row">
            <span>{label}</span>
            <div class="bar"><div style:width=move || format!("{}px", value.get() * 80.0)></div></div>
            <code>{move || format!("{:.3}ms", value.get())}</code>
        </div>
    }
}