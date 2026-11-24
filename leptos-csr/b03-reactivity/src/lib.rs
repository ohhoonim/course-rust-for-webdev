use leptos::prelude::*;
mod components;

use components::*;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <main>
            <h1>"Reactivity"</h1>
            <Counter/>
            <TemperatureConverter/>
            <WordCounter/>
            <ColorMixer/>
            <RwSignalDemo/>
        </main>
    }
}