use leptos::prelude::*;

#[derive(Clone, PartialEq)]
pub enum AvatarSize {
    Sm,
    Md,
    Lg,
}

#[component]
pub fn Avatar(
    #[prop(into)] name: String,
    #[prop(optional)] image_url: Option<String>,
    #[prop(default = AvatarSize::Md)] size: AvatarSize,
) -> impl IntoView {
    let px = match &size {
        AvatarSize::Sm => "32px",
        AvatarSize::Md => "48px",
        AvatarSize::Lg => "64px",
    };
    let font = match &size {
        AvatarSize::Sm => "0.75rem",
        AvatarSize::Md => "1rem",
        AvatarSize::Lg => "1.5rem",
    };
    let initials = name
        .split_whitespace()
        .filter_map(|word| word.chars().next())
        .take(2)
        .collect::<String>()
        .to_uppercase();

    view! {
        <div class="avatar"
            style:width=px
            style:height=px
            style:font-color=font
        >
            {if let Some(url) = image_url {
                view! { <img src=url alt=name.clone() style="width: 100%; height: 100%; object-fit: cover;"/> }.into_any()
            } else {
                view! { <span>{initials}</span> }.into_any()
            }}
        </div>
    }
}