use leptos::prelude::*;

mod components;
mod models;

use components::*; 

#[component]
pub fn App() -> impl IntoView {
    view! {
        <style>{STYLE}</style>
        <main>
            <h1>"Async and Resources"</h1>
            <PostList/>
            <UserSearch/>
            <ErrorResourceDemo/>
        </main>
    }
}

const STYLE: &str = r#"
body { margin: 0; font-family: system-ui, sans-serif; background: #f8fafc; color: #17202a; }
main { max-width: 940px; margin: 0 auto; padding: 32px 16px; }
.panel { background: white; border: 1px solid #d9dee8; border-radius: 8px; padding: 18px; margin: 14px 0; }
.row { display: flex; gap: 10px; align-items: center; flex-wrap: wrap; }
input, button { font: inherit; padding: 8px 10px; border-radius: 6px; border: 1px solid #b9c1d0; }
button { background: #1f6feb; color: white; cursor: pointer; border-color: #1f6feb; }
.cards { display: grid; gap: 10px; padding: 0; list-style: none; }
.cards article, .cards li { padding: 12px; background: #f8fafc; border-radius: 8px; border: 1px solid #e5e7eb; }
.skeletons { display: grid; gap: 8px; }
.skeletons div { height: 64px; border-radius: 8px; background: linear-gradient(90deg, #e5e7eb, #f8fafc, #e5e7eb); }
.error { background: #fef2f2; color: #991b1b; padding: 10px; border-radius: 6px; }
pre { white-space: pre-wrap; background: #111827; color: #e5e7eb; padding: 10px; border-radius: 6px; }
"#;