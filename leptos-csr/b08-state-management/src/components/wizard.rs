use leptos::prelude::*;

#[derive(Clone, Copy, PartialEq)]
pub enum WizardStep {
    Name,
    Email,
    Review,
    Done,
}

#[component]
pub fn Wizard() -> impl IntoView {
    let step = RwSignal::new(WizardStep::Name);
    let name = RwSignal::new(String::new());
    let email = RwSignal::new(String::new());
    let progress = move || match step.get() {
        WizardStep::Name => "0%",
        WizardStep::Email => "33%",
        WizardStep::Review => "66%",
        WizardStep::Done => "100%",
    };

    view! {
        <section class="panel">
            <h2>"Wizard State Machine"</h2>
            <div class="progress"><div style:width=progress></div></div>
            {move || match step.get() {
                WizardStep::Name => view! {
                    <div class="stack">
                        <input placeholder="Name" prop:value=name on:input:target=move |event| name.set(event.target().value())/>
                        <button on:click=move |_| step.set(WizardStep::Email)>"Next"</button>
                    </div>
                }.into_any(),
                WizardStep::Email => view! {
                    <div class="stack">
                        <input placeholder="Email" prop:value=email on:input:target=move |event| email.set(event.target().value())/>
                        <div class="row"><button on:click=move |_| step.set(WizardStep::Name)>"Back"</button><button on:click=move |_| step.set(WizardStep::Review)>"Review"</button></div>
                    </div>
                }.into_any(),
                WizardStep::Review => view! {
                    <div class="stack">
                        <p>{move || format!("{} <{}>", name.get(), email.get())}</p>
                        <div class="row"><button on:click=move |_| step.set(WizardStep::Email)>"Back"</button><button on:click=move |_| step.set(WizardStep::Done)>"Finish"</button></div>
                    </div>
                }.into_any(),
                WizardStep::Done => view! { <p class="success">"Done. State machine reached its terminal state."</p> }.into_any(),
            }}
        </section>
    }
}
