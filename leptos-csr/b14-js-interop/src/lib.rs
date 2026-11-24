use leptos::prelude::*;

mod components;
mod models;
use components::*;


#[component]
pub fn App() -> impl IntoView {
    view! {
        <style>{STYLE}</style>
        <main>
            <h1>"JS Interop"</h1>
            <LocalStoragePanel/>
            <ClipboardPanel/>
            <IntersectionPanel/>
            <ChartPanel/>
            <EventBusPanel/>
        </main>
    }
}

const STYLE: &str = r#"
body { margin: 0; font-family: system-ui, sans-serif; background: #f8fafc; color: #17202a; }
main { max-width: 900px; margin: 0 auto; padding: 32px 16px; }
.panel { background: white; border: 1px solid #d9dee8; border-radius: 8px; padding: 18px; margin: 14px 0; }
.row { display: flex; gap: 10px; flex-wrap: wrap; align-items: center; }
.stack { display: grid; gap: 8px; max-width: 460px; }
input, button { font: inherit; padding: 8px 10px; border-radius: 6px; border: 1px solid #94a3b8; }
button { background: #1f6feb; color: white; cursor: pointer; border-color: #1f6feb; }
button:focus-visible, input:focus-visible { outline: 3px solid #0b3d91; outline-offset: 2px; }
.spacer { height: 70vh; }
.target { margin-top: 12px; min-height: 96px; display: grid; place-items: center; border: 1px dashed #94a3b8; border-radius: 8px; background: #f8fafc; }
.chart-wrap { width: 100%; max-width: 520px; height: 220px; border: 1px solid #e5e7eb; border-radius: 8px; padding: 8px; }
canvas { width: 100%; height: 100%; }
"#;