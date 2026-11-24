use leptos::prelude::*;

#[component]
pub fn LoginForm() -> impl IntoView {
    let email = RwSignal::new(String::new());
    let password = RwSignal::new(String::new());
    let email_error = RwSignal::new(String::new());
    let password_error = RwSignal::new(String::new());
    let success = RwSignal::new(String::new());

    view! {
        <section class="panel">
            <h2>"Login Validation"</h2>
            <form class="stack" on:submit=move |event| {
                event.prevent_default();
                email_error.set(String::new());
                password_error.set(String::new());
                success.set(String::new());

                let mut valid = true;
                if !email.read().contains('@') {
                    email_error.set("Email must contain @".into());
                    valid = false;
                }
                if password.read().len() < 8 {
                    password_error.set("Password must be at least 8 characters".into());
                    valid = false;
                }
                if valid {
                    success.set(format!("Logged in as {}", email.get()));
                }
            }>
                <label>"Email" <input prop:value=email on:input:target=move |event| email.set(event.target().value())/></label>
                <small class="error-text">{email_error}</small>
                <label>"Password" <input type="password" prop:value=password on:input:target=move |event| password.set(event.target().value())/></label>
                <small class="error-text">{password_error}</small>
                <button type="submit">"Log in"</button>
            </form>
            <p class="success-text">{success}</p>
        </section>
    }
}