use leptos::prelude::*;

#[component]
pub fn LocalStoragePanel() -> impl IntoView {
    let key = RwSignal::new("course-note".to_string());
    let value = RwSignal::new(String::new());
    let status = RwSignal::new(String::from("Ready"));

    view! {
        <section class="panel">
            <h2>"localStorage CRUD"</h2>
            <div class="stack">
                <input aria-label="Key" prop:value=key on:input:target=move |event| key.set(event.target().value())/>
                <input aria-label="Value" placeholder="Value" prop:value=value on:input:target=move |event| value.set(event.target().value())/>
                <div class="row">
                    <button on:click=move |_| {
                        status.set(match save_value(&key.get_untracked(), &value.get_untracked()) {
                            Ok(()) => "Saved".to_string(),
                            Err(message) => message,
                        });
                    }>"Save"</button>
                    <button on:click=move |_| {
                        status.set(match load_value(&key.get_untracked()) {
                            Ok(Some(found)) => {
                                value.set(found);
                                "Loaded".to_string()
                            }
                            Ok(None) => "Nothing is stored under that key".to_string(),
                            Err(message) => message,
                        });
                    }>"Load"</button>
                    <button on:click=move |_| {
                        status.set(match delete_value(&key.get_untracked()) {
                            Ok(()) => {
                                value.set(String::new());
                                "Deleted".to_string()
                            }
                            Err(message) => message,
                        });
                    }>"Delete"</button>
                </div>
            </div>
            <p role="status">{status}</p>
        </section>
    }
}

fn load_value(key: &str) -> Result<Option<String>, String> {
    let key = clean_key(key)?;
    storage()?
        .get_item(key)
        .map_err(|_| "The browser refused to read that key".to_string())
}

fn delete_value(key: &str) -> Result<(), String> {
    let key = clean_key(key)?;
    storage()?
        .remove_item(key)
        .map_err(|_| "The browser refused to delete that key".to_string())
}

const MAX_VALUE_LEN: usize = 10_000;
const MAX_KEY_LEN: usize = 100;

fn save_value(key: &str, value: &str) -> Result<(), String> {
    let key = clean_key(key)?;
    if value.chars().count() > MAX_VALUE_LEN {
        return Err("That value is too long (10000 characters at most)".into());
    }
    storage()?
        .set_item(key, value)
        .map_err(|_| "The browser refused to save (storage is full or blocked)".to_string())
}

fn clean_key(raw: &str) -> Result<&str, &'static str> {
    let key = raw.trim();
    if key.is_empty() {
        Err("Type a key first")
    } else if key.chars().count() > MAX_KEY_LEN {
        Err("That key is too long (100 characters at most)")
    } else {
        Ok(key)
    }
}

fn storage() -> Result<web_sys::Storage, String> {
    let window = web_sys::window().ok_or_else(|| "There is no browser window".to_string())?;
    match window.local_storage() {
        Ok(Some(storage)) => Ok(storage),
        Ok(None) => Err("localStorage is not available in this browser".into()),
        Err(_) => Err("The browser denied access to localStorage".into()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn keys_are_trimmed_and_checked() {
        assert_eq!(clean_key("  note  "), Ok("note"));
        assert!(clean_key("").is_err());
        assert!(clean_key("   ").is_err());
        assert!(clean_key(&"k".repeat(MAX_KEY_LEN)).is_ok());
        assert!(clean_key(&"k".repeat(MAX_KEY_LEN + 1)).is_err());
        // Length counts characters, not bytes, so Hindi keys are not punished.
        assert!(clean_key(&"क".repeat(MAX_KEY_LEN)).is_ok());
    }

}