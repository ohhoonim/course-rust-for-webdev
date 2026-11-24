use leptos::prelude::*;

mod components;
mod models;
use components::*;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <style>{STYLE}</style>
        <main>
            <h1>"Testing"</h1>
            <section class="panel">
                <h2>"Component Under Test"</h2>
                <Counter/>
            </section>
            <TestReport/>
        </main>
    }
}

const STYLE: &str = r#"
body { margin: 0; font-family: system-ui, sans-serif; background: #f8fafc; color: #17202a; }
main { max-width: 860px; margin: 0 auto; padding: 32px 16px; }
.panel { background: white; border: 1px solid #d9dee8; border-radius: 8px; padding: 18px; margin: 14px 0; }
button { font: inherit; padding: 8px 10px; border-radius: 6px; border: 1px solid #1f6feb; background: #1f6feb; color: white; cursor: pointer; margin-left: 8px; }
table { width: 100%; border-collapse: collapse; }
th, td { text-align: left; border-bottom: 1px solid #e5e7eb; padding: 8px; }
.pass { color: #047857; font-weight: 800; }
.fail { color: #b91c1c; font-weight: 800; }
"#;
