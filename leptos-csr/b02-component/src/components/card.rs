use leptos::prelude::*;

#[component]
pub fn Card(
    #[prop(into)] title: String,
    #[prop(optional)] subtitle: Option<String>,
    children: Children,
) -> impl IntoView {
    view! {
        <div class="card">
            <div class="heading">
                <h3>{title}</h3>
                {subtitle.map(|s| view! { <p>{s}</p> })}
            </div>
            <div class="section">{children()}</div>
        </div>
    }
}