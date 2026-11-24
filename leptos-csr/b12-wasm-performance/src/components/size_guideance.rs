use leptos::prelude::*;

#[component]
pub fn SizeGuidance() -> impl IntoView {
    const PROFILE: &str =
        "release: opt-level=z, lto=true, codegen-units=1, panic=abort, wasm-opt=z";

    view! {
        <section class="panel">
            <h2>"WASM binary size guidance"</h2>
            <ul>
                <li>"Keep web-sys feature lists tight."</li>
                <li>"Prefer memoized derived work for repeated consumers."</li>
                <li>"Use LTO and wasm-opt for release builds."</li>
                <li>"Split rarely used UI behind routes or Suspense boundaries."</li>
            </ul>
            <code>{PROFILE}</code>
        </section>
    }
}