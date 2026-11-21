use leptos::prelude::*;

fn main() {
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(App);
}

#[component]
fn App() -> impl IntoView {
    view! {
        <h1>"Hi, There"</h1>
        <p>"I'm matthew"</p>
        <Counter initial_value=0 />
    }
}

#[component]
fn Counter(
    initial_value: i32,
    #[prop(default=1)] step: i32,
) -> impl IntoView {
    let (count, set_count) = signal(initial_value);

    let increase = move |_| set_count.update(|count| *count += step);

    view! {
        <button on:click = increase >누르면 {step}증가</button>
        <p>{count}</p>
    }
}
