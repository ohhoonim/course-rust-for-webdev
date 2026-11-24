use leptos::prelude::*;

use crate::models::TodoState;

#[component]
pub fn TestReport() -> impl IntoView {
    let cases = vec![
        ("valid email", validate_email("darshan@example.com")),
        ("invalid email", !validate_email("darshan-example")),
        ("size 0", format_file_size(0) == "0 B"),
        ("size kb", format_file_size(1024) == "1.0 KB"),
        ("todo add", {
            let mut state = TodoState::default();
            state.add("ship tests");
            state.items.len() == 1
        }),
        ("todo toggle", {
            let mut state = TodoState::default();
            let id = state.add("toggle");
            state.toggle(id);
            state.items[0].done
        }),
        ("todo delete", {
            let mut state = TodoState::default();
            let id = state.add("delete");
            state.delete(id);
            state.items.is_empty()
        }),
    ];

    view! {
        <section class="panel">
            <h2>"Visible Runtime Test Report"</h2>
            <table>
                <thead><tr><th>"Case"</th><th>"Result"</th></tr></thead>
                <tbody>
                    {cases.into_iter().map(|(name, passed)| view! {
                        <tr>
                            <td>{name}</td>
                            <td class=if passed { "pass" } else { "fail" }>{if passed { "PASS" } else { "FAIL" }}</td>
                        </tr>
                    }).collect_view()}
                </tbody>
            </table>
        </section>
    }
}

fn validate_email(input: &str) -> bool {
    let Some((local, domain)) = input.split_once('@') else {
        return false;
    };
    !local.is_empty()
        && !domain.contains('@')
        && domain.contains('.')
        && !domain.starts_with('.')
        && !domain.ends_with('.')
        && !input.contains(char::is_whitespace)
}

fn format_file_size(bytes: u64) -> String {
    const KB: f64 = 1024.0;
    const MB: f64 = KB * 1024.0;
    const GB: f64 = MB * 1024.0;

    let shown = |value: f64| (value * 10.0).round() / 10.0;
    let bytes_f = bytes as f64;
    if shown(bytes_f / MB) >= 1024.0 {
        format!("{:.1} GB", bytes_f / GB)
    } else if shown(bytes_f / KB) >= 1024.0 {
        format!("{:.1} MB", bytes_f / MB)
    } else if bytes_f >= KB {
        format!("{:.1} KB", bytes_f / KB)
    } else {
        format!("{bytes} B")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn validates_email_examples() {
        assert!(validate_email("a@b.com"));
        assert!(validate_email("darshan@example.dev"));
        assert!(!validate_email("missing-at.com"));
        assert!(!validate_email("missing-dot@example"));
        assert!(!validate_email(""));
    }

    #[test]
    fn todo_state_add_toggle_delete() {
        let mut state = TodoState::default();
        let id = state.add("write test");
        assert_eq!(state.items.len(), 1);
        state.toggle(id);
        assert!(state.items[0].done);
        state.delete(id);
        assert!(state.items.is_empty());
    }

    #[test]
    fn file_sizes_are_formatted() {
        assert_eq!(format_file_size(0), "0 B");
        assert_eq!(format_file_size(1024), "1.0 KB");
        assert_eq!(format_file_size(1024 * 1024), "1.0 MB");
        assert_eq!(format_file_size(1024 * 1024 * 1024), "1.0 GB");
    }
}

#[cfg(all(test, target_arch = "wasm32"))]
mod wasm_tests {
    use super::*;
    use wasm_bindgen_test::wasm_bindgen_test;

    wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn counter_signal_updates() {
        let count = RwSignal::new(0_i32);
        assert_eq!(count.get(), 0);
        count.set(5);
        assert_eq!(count.get(), 5);
    }
}

#[cfg(test)]
mod edge_case_tests {
    use super::*;

    #[test]
    fn ids_are_never_reused_after_a_delete() {
        let mut state = TodoState::default();
        let first = state.add("first");
        let second = state.add("second");
        state.delete(first);
        let third = state.add("third");
        assert_ne!(
            third, second,
            "a new todo must not reuse the id of a todo that is still in the list"
        );
        let mut ids: Vec<u32> = state.items.iter().map(|todo| todo.id).collect();
        let count = ids.len();
        ids.sort_unstable();
        ids.dedup();
        assert_eq!(ids.len(), count, "every id in the list must be unique");
    }

    #[test]
    fn rejects_an_email_with_nothing_before_the_at_sign() {
        assert!(!validate_email("@b.com"));
    }

    #[test]
    fn rejects_an_email_with_two_at_signs() {
        assert!(!validate_email("a@@b.com"));
    }

    #[test]
    fn rejects_domains_that_start_or_end_with_a_dot() {
        assert!(!validate_email("a@.com"));
        assert!(!validate_email("a@b."));
    }

    #[test]
    fn rejects_an_email_with_a_space() {
        assert!(!validate_email("a b@c.com"));
    }

    #[test]
    fn file_sizes_round_up_into_the_next_unit() {
        assert_eq!(format_file_size(1023), "1023 B");
        assert_eq!(format_file_size(1024), "1.0 KB");
        assert_eq!(format_file_size(1_048_575), "1.0 MB");
        assert_eq!(format_file_size(1_073_741_823), "1.0 GB");
    }
}
