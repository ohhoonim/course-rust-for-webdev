use leptos::prelude::*;

#[derive(Clone)]
pub enum BadgeColor {
    Blue,
    Green,
    Yellow,
    Red,
    Gray,
}

/// Explains the `badge` step in this lesson.
///
/// What it does: renders the `Badge` component for one focused part of the lesson UI.
/// Why it matters: Keeping this UI section isolated makes the signal, context, action, or routing pattern easier to study.
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
        <span style=format!(
            "display: inline-block; padding: 2px 8px; border-radius: 999px; \
             background: {bg}; color: {fg}; font-size: 0.78rem; font-weight: 500; margin: 2px;"
        )>
            {text}
        </span>
    }
}