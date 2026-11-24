use leptos::prelude::*;

#[component]
pub fn LazyList() -> impl IntoView {
    let visible = RwSignal::new(30_u32);
    view! {
        <section class="panel">
            <h2>"Lazy list"</h2>
            <p>"Only the rows revealed so far exist in the page. A production version would reveal more from an IntersectionObserver, or use a virtualized list that keeps just the visible rows."</p>
            <button on:click=move |_| visible.update(|value| *value = (*value + 30).min(200))>"Reveal more"</button>
            <p>{move || format!("rows in the page: {}", visible.get())}</p>
            <div class="long-list">
                // The key is the row number, so revealing more rows adds new rows and keeps the existing ones.
                <For
                    each=move || { (1..=visible.get()).collect::<Vec<_>>() }
                    key=|item| *item
                    children=|item| view! { <div class="real-row">{format!("Row {item}")}</div> }
                />
            </div>
        </section>
    }
}