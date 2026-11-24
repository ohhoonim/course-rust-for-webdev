use gloo_timers::callback::Timeout;
use leptos::prelude::*;

#[component]
pub fn FadeInOut() -> impl IntoView {
    let visible = RwSignal::new(true);
    let mounted = RwSignal::new(true);
    let generation = RwSignal::new(0_u32);
    view! {
        <section class="panel">
            <h2>"FadeInOut with deferred removal"</h2>
            <button on:click=move |_| {
                let current = generation.get_untracked() + 1;
                generation.set(current);
                if visible.get_untracked() {
                    visible.set(false);
                    Timeout::new(420, move || {
                        if generation.try_get_untracked() == Some(current) {
                            mounted.set(false);
                        }
                    })
                    .forget();
                } else {
                    // Mount first at opacity zero, then fade in on the next frames.
                    mounted.set(true);
                    Timeout::new(30, move || {
                        if generation.try_get_untracked() == Some(current) {
                            visible.set(true);
                        }
                    })
                    .forget();
                }
            }>"Toggle"</button>
            <Show when=move || mounted.get()>
                <div
                    class="fade-box"
                    style:opacity=move || if visible.get() { "1" } else { "0" }
                    style:transition="opacity 0.4s ease"
                >
                    "The node stays mounted until the exit animation completes."
                </div>
            </Show>
        </section>
    }
}
