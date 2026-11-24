use leptos::prelude::*;

#[component]
pub fn UserCard(
    #[prop(into)] name: String,
    #[prop(optional)] avatar_url: Option<String>,
) -> impl IntoView {
    view! {
        <article class="user-card">
            {avatar_url
                .map(|url| view! { <img class="avatar" src=url alt=name.clone()/> }.into_any())
                .unwrap_or_else(|| view! { <div class="avatar placeholder">{name.chars().next().unwrap_or('?')}</div> }.into_any())}
            <div>
                <strong>{name}</strong>
                <p>"Option rendering keeps missing media explicit."</p>
            </div>
        </article>
    }
}