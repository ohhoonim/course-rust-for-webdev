use leptos::prelude::* ; 

#[component]
pub fn PreferenceForm() -> impl IntoView {
    let email = RwSignal::new(true);
    let sms = RwSignal::new(false);
    let product = RwSignal::new(true);
    let density = RwSignal::new("comfortable".to_string());
    let timezone = RwSignal::new("UTC".to_string());

    view! {
        <section class="panel">
            <h2>"Preferences"</h2>
            <div class="grid">
                <fieldset>
                    <legend>"Notifications"</legend>
                    <label><input type="checkbox" prop:checked=email on:change:target=move |event| email.set(event.target().checked())/>" Email"</label>
                    <label><input type="checkbox" prop:checked=sms on:change:target=move |event| sms.set(event.target().checked())/>" SMS"</label>
                    <label><input type="checkbox" prop:checked=product on:change:target=move |event| product.set(event.target().checked())/>" Product updates"</label>
                </fieldset>
                <fieldset>
                    <legend>"Density"</legend>
                    {["compact", "comfortable", "spacious"].into_iter().map(|choice| view! {
                        <label>
                            <input
                                type="radio"
                                name="density"
                                value=choice
                                prop:checked=move || density.get() == choice
                                on:change=move |_| density.set(choice.to_string())
                            />
                            {format!(" {choice}")}
                        </label>
                    }).collect_view()}
                </fieldset>
                <label>
                    "Timezone"
                    <select prop:value=timezone on:change:target=move |event| timezone.set(event.target().value())>
                        <option value="UTC">"UTC"</option>
                        <option value="Asia/Kolkata">"Asia/Kolkata"</option>
                        <option value="America/New_York">"America/New_York"</option>
                    </select>
                </label>
            </div>
            <div class="summary">
                {move || format!(
                    "Email: {}, SMS: {}, Product: {}, Density: {}, Timezone: {}",
                    email.get(),
                    sms.get(),
                    product.get(),
                    density.get(),
                    timezone.get()
                )}
            </div>
        </section>
    }
}