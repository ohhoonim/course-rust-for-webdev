use gloo_timers::callback::Interval;
use leptos::prelude::*;

#[component]
pub fn BouncingBall() -> impl IntoView {
    let running = RwSignal::new(false);
    let y = RwSignal::new(0.0_f64);
    let velocity = RwSignal::new(0.0_f64);

    // Keep the timer in a local store and drop it when the component goes away. With `.forget()` the
    // timer would run forever, and mounting this component again would start a second one.
    let timer = StoredValue::new_local(Some(Interval::new(16, move || {
        if !running.get_untracked() {
            return;
        }
        velocity.update(|value| *value += 0.8);
        y.update(|position| {
            *position += velocity.get_untracked();
            if *position > 170.0 {
                *position = 170.0;
                velocity.update(|value| *value *= -0.72);
            }
        });
    })));
    on_cleanup(move || timer.set_value(None));

    view! {
        <section class="panel">
            <h2>"BouncingBall loop"</h2>
            <button on:click=move |_| {
                y.set(0.0);
                velocity.set(0.0);
                running.set(true);
            }>"Drop ball"</button>
            <button on:click=move |_| running.set(false)>"Pause"</button>
            <div class="stage">
                <div class="ball" style:transform=move || format!("translateY({:.1}px)", y.get())></div>
            </div>
        </section>
    }
}