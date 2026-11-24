use leptos::prelude::*;

use crate::models::use_lang;

#[component]
pub fn KeyboardMenu() -> impl IntoView {
    let lang = use_lang();
    let selected = RwSignal::new(0_usize);
    let items = ["Overview", "Lessons", "Community"];
    let refs: [NodeRef<leptos::html::Button>; 3] = [NodeRef::new(), NodeRef::new(), NodeRef::new()];

    view! {
        <section class="panel">
            <h2>"Keyboard-friendly menu"</h2>
            <div role="menubar" aria-label="Main menu" class="menu">
                {items.into_iter().enumerate().map(|(index, item)| view! {
                    <button
                        node_ref=refs[index]
                        role="menuitem"
                        tabindex=move || if selected.get() == index { "0" } else { "-1" }
                        aria-current=move || if selected.get() == index { "true" } else { "false" }
                        class:active=move || selected.get() == index
                        on:click=move |_| selected.set(index)
                        on:keydown=move |event: web_sys::KeyboardEvent| {
                            let rtl = lang.get_untracked().dir() == "rtl";
                            if let Some(next) = next_menu_index(index, items.len(), &event.key(), rtl) {
                                event.prevent_default();
                                selected.set(next);
                                if let Some(button) = refs[next].get() {
                                    let _ = button.focus();
                                }
                            }
                        }
                    >
                        {item}
                    </button>
                }).collect_view()}
            </div>
        </section>
    }
}

fn next_menu_index(current: usize, len: usize, key: &str, rtl: bool) -> Option<usize> {
    if len == 0 {
        return None;
    }
    let forward = (current + 1) % len;
    let backward = (current + len - 1) % len;
    match key {
        "ArrowRight" => Some(if rtl { backward } else { forward }),
        "ArrowLeft" => Some(if rtl { forward } else { backward }),
        "Home" => Some(0),
        "End" => Some(len - 1),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

     #[test]
    fn arrows_move_and_wrap() {
        assert_eq!(next_menu_index(0, 3, "ArrowRight", false), Some(1));
        assert_eq!(next_menu_index(2, 3, "ArrowRight", false), Some(0));
        assert_eq!(next_menu_index(0, 3, "ArrowLeft", false), Some(2));
    }

    #[test]
    fn arrows_follow_the_eye_in_right_to_left() {
        assert_eq!(next_menu_index(0, 3, "ArrowLeft", true), Some(1));
        assert_eq!(next_menu_index(0, 3, "ArrowRight", true), Some(2));
    }

    #[test]
    fn home_end_and_other_keys() {
        assert_eq!(next_menu_index(1, 3, "Home", false), Some(0));
        assert_eq!(next_menu_index(1, 3, "End", false), Some(2));
        assert_eq!(next_menu_index(1, 3, "a", false), None);
        assert_eq!(next_menu_index(0, 0, "ArrowRight", false), None);
    }
}