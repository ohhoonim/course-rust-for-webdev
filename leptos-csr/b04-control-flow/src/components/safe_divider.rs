use leptos::prelude::*; 
use std::error::Error;
use std::fmt;

#[derive(Debug, Clone)]
struct DivideError(String);

impl fmt::Display for DivideError {
    /// Explains the `fmt` step in this lesson.
    ///
    /// What it does: implements the `fmt` operation for this lesson data type or local helper.
    /// Why it matters: Method-sized behavior keeps state transitions small enough to test and explain.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Error for DivideError {}

/// Explains the `safe divider` step in this lesson.
///
/// What it does: renders the `SafeDivider` component for one focused part of the lesson UI.
/// Why it matters: Keeping this UI section isolated makes the signal, context, action, or routing pattern easier to study.
#[component]
pub fn SafeDivider() -> impl IntoView {
    let a = RwSignal::new(10.0_f64);
    let b = RwSignal::new(2.0_f64);
    let result = RwSignal::new(Ok::<f64, DivideError>(5.0));

    view! {
        <section class="panel">
            <h2>"SafeDivider - ErrorBoundary"</h2>
            <div class="row">
                <input
                    type="number"
                    prop:value=move || a.get().to_string()
                    on:input:target=move |event| {
                        if let Ok(value) = event.target().value().parse() {
                            a.set(value);
                        }
                    }
                />
                <span>"/"</span>
                <input
                    type="number"
                    prop:value=move || b.get().to_string()
                    on:input:target=move |event| {
                        if let Ok(value) = event.target().value().parse() {
                            b.set(value);
                        }
                    }
                />
                <button on:click=move |_| {
                    if b.get() == 0.0 {
                        result.set(Err(DivideError("division by zero".into())));
                    } else {
                        result.set(Ok(a.get() / b.get()));
                    }
                }>"Divide"</button>
            </div>
            <ErrorBoundary fallback=|errors| view! {
                <div class="error">
                    <strong>"Caught error"</strong>
                    <ul>{move || errors.get().into_iter().map(|(_, error)| view! { <li>{error.to_string()}</li> }).collect_view()}</ul>
                </div>
            }>
                {move || result.get().map(|value| view! { <p class="success">"Result: " {format!("{value:.2}")}</p> })}
            </ErrorBoundary>
        </section>
    }
}