use leptos::prelude::*;

#[component]
pub fn LoginGate() -> impl IntoView {
    let logged_in = RwSignal::new(false);
    let email = RwSignal::new(String::new());
    let password = RwSignal::new(String::new());

    view! {
        <section class="panel">
            <h2>"LoginGate - <Show/>"</h2>
            <Show
                when=move || logged_in.get()
                fallback=move || view! {
                    <form class="stack" on:submit=move |event| {
                        event.prevent_default();
                        if email.read().contains('@') && password.read().len() >= 4 {
                            logged_in.set(true);
                        }
                    }>
                        <input
                            type="email"
                            placeholder="email"
                            prop:value=email
                            on:input:target=move |event| email.set(event.target().value())
                        />
                        <input
                            type="password"
                            placeholder="password"
                            prop:value=password
                            on:input:target=move |event| password.set(event.target().value())
                        />
                        <button type="submit">"Log in"</button>
                    </form>
                }
            >
                <div class="success">
                    <strong>"Welcome back."</strong>
                    <p>{move || format!("Signed in as {}", email.get())}</p>
                    <button on:click=move |_| logged_in.set(false)>"Log out"</button>
                </div>
            </Show>
        </section>
    }
}