mod components;

use components::*;
use leptos::prelude::*;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <main>
            <h1>"Control Flow"</h1>
            <LoginGate/>
            <TodoList/>
            <TabPanel/>
            <section class="panel">
                <h2>"UserCard - Option rendering"</h2>
                <UserCard name="Aarambh Dev Hub"/>
                <UserCard name="Leptos Learner" avatar_url="https://placehold.co/96x96".to_string()/>
            </section>
            <SafeDivider/>
        </main>
    }
}
