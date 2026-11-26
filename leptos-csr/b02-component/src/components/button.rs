use leptos::prelude::*;

#[derive(Clone, PartialEq)]
pub enum ButtonVariant {
    Primary,
    Secondary,
    Danger,
    Ghost,
}

#[component]
pub fn Button(
    #[prop(into)] label: String,
    #[prop(default = ButtonVariant::Primary)] variant: ButtonVariant,
    #[prop(default = false)] disabled: bool,
    #[prop(optional)] on_click: Option<Callback<web_sys::MouseEvent>>,
) -> impl IntoView {
    let (bg, text_color) = match &variant {
        ButtonVariant::Primary => ("#1d4ed8", "white"),
        ButtonVariant::Secondary => ("#6b7280", "white"),
        ButtonVariant::Danger => ("#dc2626", "white"),
        ButtonVariant::Ghost => ("transparent", "#374151"),
    };

    view! {
        <button class="btn-command"
            style:background=bg
            style:color=text_color
            style:border-color=bg
            disabled=disabled
            on:click=move |event| {
                if let Some(callback) = &on_click {
                    callback.run(event);
                }
            }
        >
            {label}
        </button>
    }
}
