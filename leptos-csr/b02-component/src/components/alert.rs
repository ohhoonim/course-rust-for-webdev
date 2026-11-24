use leptos::prelude::*;

#[derive(Clone, PartialEq)]
pub enum AlertType {
    Info,
    Success,
    Warning,
    Error,
}

#[component]
pub fn Alert(
    #[prop(into)] message: String,
    #[prop(default = AlertType::Info)] alert_type: AlertType,
    #[prop(default = false)] dismissible: bool,
) -> impl IntoView {
    let (dismissed, set_dismissed) = signal(false);
    let (border, bg, icon) = match &alert_type {
        AlertType::Info => ("#3b82f6", "#eff6ff", "i"),
        AlertType::Success => ("#22c55e", "#f0fdf4", "ok"),
        AlertType::Warning => ("#f59e0b", "#fffbeb", "!"),
        AlertType::Error => ("#ef4444", "#fef2f2", "x"),
    };

    view! {
        <Show when=move || !dismissed.get()>
            <div style=format!(
                "display: flex; align-items: center; padding: 10px 14px; background: {bg}; \
                 border-left: 4px solid {border}; border-radius: 6px; margin: 8px 0; gap: 8px;"
            )>
                <span>{icon}</span>
                <span style="flex: 1;">{message.clone()}</span>
                {dismissible.then(|| view! {
                    <button
                        on:click=move |_| set_dismissed.set(true)
                        style="background: none; border: none; cursor: pointer; font-size: 1rem; color: #6b7280;"
                    >
                        "x"
                    </button>
                })}
            </div>
        </Show>
    }
}
