use leptos::prelude::*;

#[component]
pub fn GradientPreview() -> impl IntoView {
    let start = RwSignal::new("#2563eb".to_string());
    let end = RwSignal::new("#f97316".to_string());

    view! {
        <section class="panel">
            <h2>"Dynamic gradient styles"</h2>
            <div class="row">
                <input type="color" prop:value=start on:input:target=move |event| start.set(event.target().value())/>
                <input type="color" prop:value=end on:input:target=move |event| end.set(event.target().value())/>
            </div>
            <div class="gradient" style:background=move || format!("linear-gradient(135deg, {}, {})", start.get(), end.get())>
                {move || format!("{} -> {}", start.get(), end.get())}
            </div>
        </section>
    }
}
