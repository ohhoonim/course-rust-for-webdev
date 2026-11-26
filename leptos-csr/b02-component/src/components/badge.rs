use leptos::prelude::*;

#[derive(Clone)]
pub enum BadgeColor {
    Blue,
    Green,
    Yellow,
    Red,
    Gray,
}

#[component]
pub fn Badge(
    #[prop(into)] text: String,
    #[prop(default = BadgeColor::Blue)] color: BadgeColor,
) -> impl IntoView {
    let (bg, fg) = match &color {
        BadgeColor::Blue => ("#dbeafe", "#1e40af"),
        BadgeColor::Green => ("#dcfce7", "#166534"),
        BadgeColor::Yellow => ("#fef9c3", "#854d0e"),
        BadgeColor::Red => ("#fee2e2", "#991b1b"),
        BadgeColor::Gray => ("#f3f4f6", "#374151"),
    };

    view! {
        <span class="badge"
            style:background=bg 
            style:color=fg
        >
            {text}
        </span>
    }
}