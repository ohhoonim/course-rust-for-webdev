# 기초 실습 예제: Counter

```rust
use leptos::prelude::*;

#[component]
pub fn SimpleCounter(
    initial_value: i32,
    step: i32,
) -> impl IntoView {
    let (value, set_value) = signal(initial_value);

    view! {
        <div>
            <button on:click=move |_| set_value.set(0)>"Clear"</button>
            <button on:click=move |_| *set_value.write() -= step>"-1"</button>
            <span>"Value: " {value} "!"</span>
            <button on:click=move |_| set_value.update(|value| *value += step)>"+1"</button>
        </div>
    }
}

pub fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(|| {
        view! { <SimpleCounter initial_value=0 step=1/> }
    })
}
```