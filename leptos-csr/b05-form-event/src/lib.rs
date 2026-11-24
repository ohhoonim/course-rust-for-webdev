mod components;

use components::*; 
use leptos::prelude::*;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <style>{STYLE}</style>
        <main>
            <h1>"Forms 과 Events"</h1>
            <LiveTextInput/>
            <LoginForm/>
            <PreferenceForm/>
            <MultiStepForm/>
            <SearchNodeRef/>
        </main>
    }
}

const STYLE: &str = r#"
body { margin: 0; font-family: system-ui, sans-serif; background: #f8fafc; color: #17202a; }
main { max-width: 900px; margin: 0 auto; padding: 32px 16px; }
.panel { background: white; border: 1px solid #d9dee8; border-radius: 8px; padding: 18px; margin: 14px 0; }
.stack { display: grid; gap: 10px; max-width: 430px; }
.row { display: flex; gap: 10px; align-items: center; flex-wrap: wrap; }
.grid { display: grid; grid-template-columns: repeat(auto-fit, minmax(220px, 1fr)); gap: 16px; }
input, button, select, textarea { font: inherit; padding: 8px 10px; border-radius: 6px; border: 1px solid #b9c1d0; }
button { background: #1f6feb; color: white; cursor: pointer; border-color: #1f6feb; }
label { display: grid; gap: 4px; margin: 4px 0; }
fieldset label { display: block; }
.preview, .summary { background: #f8fafc; border-radius: 6px; padding: 10px; }
.error-text { color: #b91c1c; min-height: 1rem; }
.success-text { color: #047857; font-weight: 600; }
.progress { height: 10px; background: #e5e7eb; border-radius: 999px; overflow: hidden; margin-bottom: 12px; }
.progress div { height: 100%; background: #1f6feb; transition: width 0.2s ease; }
"#;