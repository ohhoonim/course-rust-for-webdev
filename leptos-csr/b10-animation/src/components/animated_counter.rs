use std::time::Duration;

use gloo_timers::future::sleep;
use leptos::prelude::*;

#[component]
pub fn AnimatedCounter() -> impl IntoView {
    let target = RwSignal::new(0.0_f64);
    let display = RwSignal::new(0.0_f64);
    // Each animation gets a number. An older loop stops as soon as a newer one starts,
    // so two loops never fight over `display`.
    let latest_run = RwSignal::new(0_u32);
    let animate_to = move |next: f64| {
        target.set(next);
        let from = display.get_untracked();
        let run_id = latest_run.get_untracked() + 1;
        latest_run.set(run_id);
        leptos::task::spawn_local(async move {
            for step in 1..=24 {
                sleep(Duration::from_millis(16)).await;
                if latest_run.try_get_untracked() != Some(run_id) {
                    return;
                }
                let t = step as f64 / 24.0;
                display.set(from + (next - from) * t);
            }
        });
    };
    view! {
        <section class="panel">
            <h2>"AnimatedCounter"</h2>
            <p class="big-number">{move || format!("{:.0}", display.get())}</p>
            <button on:click=move |_| animate_to(target.get() + 25.0)>"+25"</button>
            <button on:click=move |_| animate_to(0.0)>"Reset"</button>
        </section>
    }
}
