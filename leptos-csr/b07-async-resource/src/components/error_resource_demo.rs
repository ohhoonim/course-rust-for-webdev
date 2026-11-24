use leptos::prelude::*;

use crate::models::fetch_maybe_bad; 


#[component]
pub fn ErrorResourceDemo() -> impl IntoView {
    let use_bad_url = RwSignal::new(false);
    let body = LocalResource::new(move || fetch_maybe_bad(use_bad_url.get()));

    view! {
        <section class="panel">
            <h2>"Error Resource"</h2>
            <label>
                <input
                    type="checkbox"
                    prop:checked=use_bad_url
                    on:change:target=move |event| use_bad_url.set(event.target().checked())
                />
                " Use invalid URL"
            </label>
            <Suspense fallback=|| view! { <p>"Loading endpoint..."</p> }>
                {move || body.get().map(|result| match result {
                    Ok(text) => view! { <pre>{text.chars().take(240).collect::<String>()}</pre> }.into_any(),
                    Err(error) => view! {
                        <div class="error">
                            <strong>"Fetch failed"</strong>
                            <p>{error}</p>
                        </div>
                    }.into_any(),
                })}
            </Suspense>
        </section>
    }
}