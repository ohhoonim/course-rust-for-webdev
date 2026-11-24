use leptos::prelude::*;

#[component]
pub fn StaggeredList() -> impl IntoView {
    let run = RwSignal::new(0_u32);
    let items = ["Signals", "Effects", "Memos", "Resources", "Actions"];
    view! {
        <section class="panel">
            <h2>"StaggeredList"</h2>
            <button on:click=move |_| run.update(|value| *value += 1)>"Replay"</button>
            // A CSS animation only restarts on a brand new element. A closure that returns the same view shape
            // is patched in place, so use a keyed list whose key changes on every replay: the old list is
            // removed and a new one is created.
            <For
                each=move || vec![run.get()]
                key=|replay| *replay
                children=move |_| view! {
                    <ul class="staggered">
                        {items.into_iter().enumerate().map(|(index, item)| view! {
                            <li style=format!("animation-delay: {}ms", index * 100)>{item}</li>
                        }).collect_view()}
                    </ul>
                }
            />
        </section>
    }
}

