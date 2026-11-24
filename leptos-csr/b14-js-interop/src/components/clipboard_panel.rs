use leptos::prelude::*;
use wasm_bindgen::JsValue;

use crate::models::describe_error;

#[component]
pub fn ClipboardPanel() -> impl IntoView {
    let text = RwSignal::new("Copied from Rust/WASM".to_string());
    let status = RwSignal::new(String::from("Clipboard idle"));

    view! {
        <section class="panel">
            <h2>"navigator.clipboard (async API)"</h2>
            <input aria-label="Text to copy" prop:value=text on:input:target=move |event| text.set(event.target().value())/>
            <button on:click=move |_| {
                let value = text.get_untracked();
                status.set("Copying...".into());
                leptos::task::spawn_local(async move {
                    let message = match copy_to_clipboard(&value).await {
                        Ok(()) => "Copied".to_string(),
                        Err(message) => message,
                    };
                    let _ = status.try_set(message);
                });
            }>"Copy"</button>
            <p role="status">{status}</p>
        </section>
    }
}

async fn copy_to_clipboard(text: &str) -> Result<(), String> {
    let window = web_sys::window().ok_or_else(|| "There is no browser window".to_string())?;
    let navigator = window.navigator();
    let available = js_sys::Reflect::get(&navigator, &JsValue::from_str("clipboard"))
        .map(|value| !value.is_undefined() && !value.is_null())
        .unwrap_or(false);
    if !available {
        return Err("The clipboard needs a secure page (https or localhost)".into());
    }
    wasm_bindgen_futures::JsFuture::from(navigator.clipboard().write_text(text))
        .await
        .map(|_| ())
        .map_err(|error| format!("The browser blocked the copy ({})", describe_error(&error)))
}

