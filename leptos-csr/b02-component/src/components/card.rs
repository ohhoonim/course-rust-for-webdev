use leptos::prelude::*;

#[component]
pub fn Card(
    #[prop(into)] title: String,
    #[prop(optional)] subtitle: Option<String>,
    children: Children,
) -> impl IntoView {
    view! {
        <div style="border: 1px solid #e5e7eb; border-radius: 8px; overflow: hidden; margin: 8px 0;">
            <div style="padding: 12px 16px; background: #f9fafb; border-bottom: 1px solid #e5e7eb;">
                <h3 style="margin: 0 0 2px; font-size: 1rem;">{title}</h3>
                {subtitle.map(|s| view! { <p style="margin: 0; font-size: 0.85rem; color: #6b7280;">{s}</p> })}
            </div>
            <div style="padding: 16px;">{children()}</div>
        </div>
    }
}