use leptos::prelude::*;

#[component]
pub fn TemperatureConverter() -> impl IntoView {
    let (celsius, set_celsius) = signal(0.0_f64);
    let fahrenheit = move || celsius.get() * 9.0 / 5.0 + 32.0;

    view! {
        <section>
            <h3>"온도 변환기 - derived signal"</h3>
            <div class="temperature" >
                <label>
                    "섭씨(Celsius): "
                    <input
                        type="number"
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
                    "화씨(Fahrenheit): "
                    <input
                        type="number"
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