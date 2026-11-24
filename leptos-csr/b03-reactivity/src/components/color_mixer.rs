use leptos::prelude::*;

#[component]
pub fn ColorMixer() -> impl IntoView {
    let (r, set_r) = signal(128_u8);
    let (g, set_g) = signal(64_u8);
    let (b, set_b) = signal(200_u8);
    let color_css = move || format!("rgb({}, {}, {})", r.get(), g.get(), b.get());
    let hex = move || format!("#{:02X}{:02X}{:02X}", r.get(), g.get(), b.get());

    let slider = |label: &'static str, value: ReadSignal<u8>, setter: WriteSignal<u8>| {
        view! {
            <label style="display: flex; align-items: center; gap: 8px; margin: 4px 0;">
                {label}
                <input
                    type="range"
                    min="0"
                    max="255"
                    prop:value=move || value.get().to_string()
                    on:input:target=move |event| {
                        if let Ok(next) = event.target().value().parse::<u8>() {
                            setter.set(next);
                        }
                    }
                    style="flex: 1;"
                />
                <code style="width: 2.5rem; text-align: right;">{move || value.get().to_string()}</code>
            </label>
        }
    };

    view! {
        <section style="padding: 1rem; border: 1px solid #e2e8f0; border-radius: 8px; margin: 8px 0;">
            <h3>"Color Mixer - derived signal from three sources"</h3>
            {slider("R", r, set_r)}
            {slider("G", g, set_g)}
            {slider("B", b, set_b)}
            <div style=move || format!(
                "margin-top: 12px; height: 60px; border-radius: 8px; background: {}; \
                 display: flex; align-items: center; justify-content: center; \
                 font-weight: 600; color: white; text-shadow: 0 1px 2px rgba(0,0,0,0.5);",
                color_css()
            )>
                {hex}
            </div>
        </section>
    }
}