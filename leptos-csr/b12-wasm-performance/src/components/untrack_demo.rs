use leptos::prelude::*;

#[component]
pub fn UntrackDemo() -> impl IntoView {
    let a = RwSignal::new(0_i32);
    let b = RwSignal::new(0_i32);
    let normal_runs = RwSignal::new(0_u32);
    let untracked_runs = RwSignal::new(0_u32);

    Effect::new(move |_| {
        let _ = (a.get(), b.get());
        normal_runs.update(|value| *value += 1);
    });
    Effect::new(move |_| {
        let _ = a.get();
        let _ = b.get_untracked();
        untracked_runs.update(|value| *value += 1);
    });

    view! {
        <section class="panel">
            <h2>"Untracked reads"</h2>
            <button on:click=move |_| a.update(|value| *value += 1)>"Update A"</button>
            <button on:click=move |_| b.update(|value| *value += 1)>"Update B"</button>
            <p>{move || format!("a={} b={}", a.get(), b.get())}</p>
            <p>{move || format!("normal effect runs: {}", normal_runs.get())}</p>
            <p>{move || format!("untracked-B effect runs: {}", untracked_runs.get())}</p>
        </section>
    }
}