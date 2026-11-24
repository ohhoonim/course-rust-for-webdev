use leptos::prelude::*;

#[component]
pub fn TemperatureConverter() -> impl IntoView {
    let (celsius, set_celsius) = signal(0.0_f64);
    let fahrenheit = move || celsius.get() * 9.0 / 5.0 + 32.0;

    view! {
        <section style="padding: 1rem; border: 1px solid #e2e8f0; border-radius: 8px; margin: 8px 0;">
            <h3>"Temperature Converter - derived signal"</h3>
            <div style="display: flex; gap: 16px; align-items: center; flex-wrap: wrap;">
                <label>
                    "Celsius: "
                    <input
                        type="number"
                        style="width: 80px; padding: 4px;"
                        prop:value=move || celsius.get().to_string()
                        on:input:target=move |event| {
                            if let Ok(value) = event.target().value().parse::<f64>() {
                                set_celsius.set(value);
                            }
                        }
                    />
                </label>
                <span>"="</span>
                <label>
                    "Fahrenheit: "
                    <input
                        type="number"
                        style="width: 80px; padding: 4px;"
                        prop:value=move || format!("{:.1}", fahrenheit())
                        on:input:target=move |event| {
                            if let Ok(value) = event.target().value().parse::<f64>() {
                                set_celsius.set((value - 32.0) * 5.0 / 9.0);
                            }
                        }
                    />
                </label>
            </div>
        </section>
    }
}