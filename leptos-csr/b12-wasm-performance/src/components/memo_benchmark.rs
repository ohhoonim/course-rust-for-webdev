use std::sync::atomic::{AtomicUsize, Ordering};

use leptos::prelude::*;

static MEMO_FIB_CALLS: AtomicUsize = AtomicUsize::new(0);
static RAW_FIB_CALLS: AtomicUsize = AtomicUsize::new(0);

#[component]
pub fn MemoBenchmark() -> impl IntoView {
    let input = RwSignal::new(30_u32);
    let memo_fib = Memo::new(move |_| {
        MEMO_FIB_CALLS.fetch_add(1, Ordering::Relaxed);
        fib(input.get())
    });
    let raw_fib = move || {
        RAW_FIB_CALLS.fetch_add(1, Ordering::Relaxed);
        fib(input.get())
    };
    // The call counters are plain atomics, which are not reactive. Copy them into a signal after each
    // input change (once the page has updated) so the numbers on screen stay current.
    let counts = RwSignal::new((0_usize, 0_usize));
    Effect::new(move |_| {
        input.track();
        request_animation_frame(move || {
            counts.set((
                MEMO_FIB_CALLS.load(Ordering::Relaxed),
                RAW_FIB_CALLS.load(Ordering::Relaxed),
            ));
        });
    });

    view! {
        <section class="panel">
            <h2>"Memo vs derived signal"</h2>
            <input
                type="number"
                min="1"
                max="35"
                prop:value=move || input.get().to_string()
                on:input:target=move |event| {
                    if let Ok(value) = event.target().value().parse::<u32>() {
                        input.set(clamp_fib_input(value));
                    }
                }
            />
            <div class="grid">
                <div>
                    <h3>"Memo consumers"</h3>
                    {(0..10).map(|index| view! { <p>{index} ": " {memo_fib}</p> }).collect_view()}
                </div>
                <div>
                    <h3>"Derived consumers"</h3>
                    {(0..10).map(|index| view! { <p>{index} ": " {raw_fib}</p> }).collect_view()}
                </div>
            </div>
            <p>{move || {
                let (memo_calls, derived_calls) = counts.get();
                format!("memo calls: {memo_calls}, derived calls: {derived_calls}")
            }}</p>
        </section>
    }
}

fn fib(n: u32) -> u64 {
    match n {
        0 => 0,
        1 => 1,
        n => fib(n - 1) + fib(n - 2),
    }
}

const MAX_FIB_INPUT: u32 = 35;

fn clamp_fib_input(value: u32) -> u32 {
    value.clamp(1, MAX_FIB_INPUT)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fib_matches_known_values() {
        assert_eq!(fib(0), 0);
        assert_eq!(fib(1), 1);
        assert_eq!(fib(10), 55);
        assert_eq!(fib(20), 6765);
    }

    #[test]
    fn typed_input_is_clamped_to_a_safe_range() {
        assert_eq!(clamp_fib_input(0), 1);
        assert_eq!(clamp_fib_input(30), 30);
        assert_eq!(clamp_fib_input(35), 35);
        assert_eq!(clamp_fib_input(45), 35);
        assert_eq!(clamp_fib_input(u32::MAX), 35);
    }
}