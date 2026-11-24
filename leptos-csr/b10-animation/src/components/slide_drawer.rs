use leptos::prelude::*;

#[component]
pub fn SlideDrawer() -> impl IntoView {
    let open = RwSignal::new(false);

    view! {
        <section class="panel drawer-demo">
            <h2>"SlideDrawer"</h2>
            <button on:click=move |_| open.set(true)>"Open drawer"</button>
            <div
                class="overlay"
                style:opacity=move || if open.get() { "1" } else { "0" }
                style:pointer-events=move || if open.get() { "auto" } else { "none" }
                on:click=move |_| open.set(false)
            ></div>
            <aside
                class="drawer"
                style:transform=move || if open.get() { "translateX(0)" } else { "translateX(-110%)" }
            >
                <h3>"Navigation"</h3>
                <p>"Transform transitions are GPU-friendly and predictable."</p>
                <button on:click=move |_| open.set(false)>"Close"</button>
            </aside>
        </section>
    }
}
