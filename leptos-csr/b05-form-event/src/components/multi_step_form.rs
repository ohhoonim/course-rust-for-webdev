use leptos::prelude::*;

#[component]
pub fn MultiStepForm() -> impl IntoView {
    let step = RwSignal::new(1_u8);
    let name = RwSignal::new(String::new());
    let email = RwSignal::new(String::new());
    let password = RwSignal::new(String::new());
    let confirm = RwSignal::new(String::new());
    let submitted = RwSignal::new(false);
    let progress = move || format!("{}%", (step.get() as u32 - 1) * 50);

    view! {
        <section class="panel">
            <h2>"Multi-step Form"</h2>
            <div class="progress"><div style:width=progress></div></div>
            {move || if step.get() == 1 {
                view! {
                    <div class="stack">
                        <input placeholder="Name" prop:value=name on:input:target=move |event| name.set(event.target().value())/>
                        <input placeholder="Email" prop:value=email on:input:target=move |event| email.set(event.target().value())/>
                        <button on:click=move |_| step.set(2)>"Next"</button>
                    </div>
                }.into_any()
            } else if step.get() == 2 {
                view! {
                    <div class="stack">
                        <input type="password" placeholder="Password" prop:value=password on:input:target=move |event| password.set(event.target().value())/>
                        <input type="password" placeholder="Confirm password" prop:value=confirm on:input:target=move |event| confirm.set(event.target().value())/>
                        <div class="row"><button on:click=move |_| step.set(1)>"Back"</button><button on:click=move |_| step.set(3)>"Review"</button></div>
                    </div>
                }.into_any()
            } else {
                view! {
                    <div class="stack">
                        <p>{move || format!("Name: {} | Email: {}", name.get(), email.get())}</p>
                        <p>{move || if password.get() == confirm.get() { "Passwords match" } else { "Passwords do not match" }}</p>
                        <div class="row">
                            <button on:click=move |_| step.set(2)>"Back"</button>
                            <button on:click=move |_| submitted.set(true)>"Submit"</button>
                        </div>
                        <p class="success-text">{move || submitted.get().then_some("Submitted")}</p>
                    </div>
                }.into_any()
            }}
        </section>
    }
}
