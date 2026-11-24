use leptos::prelude::*;

mod components;
use components::*;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <style>{STYLE}</style>
        <main>
            <h1>"WASM Performance"</h1>
            <BatchingBenchmark/>
            <MemoBenchmark/>
            <UntrackDemo/>
            <LazyList/>
            <SizeGuidance/>
        </main>
    }
}

const STYLE: &str = r#"
body { margin: 0; font-family: system-ui, sans-serif; background: #f8fafc; color: #17202a; }
main { max-width: 960px; margin: 0 auto; padding: 32px 16px; }
.panel { background: white; border: 1px solid #d9dee8; border-radius: 8px; padding: 18px; margin: 14px 0; }
button, input { font: inherit; padding: 8px 10px; border-radius: 6px; border: 1px solid #1f6feb; margin: 3px; }
button { background: #1f6feb; color: white; cursor: pointer; }
.grid { display: grid; grid-template-columns: repeat(auto-fit, minmax(260px, 1fr)); gap: 18px; }
.bar-row { display: grid; grid-template-columns: 90px 1fr 80px; gap: 10px; align-items: center; margin: 8px 0; }
.bar { height: 12px; border-radius: 999px; background: #e5e7eb; overflow: hidden; }
.bar div { height: 100%; min-width: 4px; background: #1f6feb; }
.long-list { max-height: 340px; overflow: auto; display: grid; gap: 6px; }
.real-row, .placeholder-row { min-height: 34px; border-radius: 6px; padding: 8px; }
.real-row { background: #ecfdf5; color: #065f46; }
.placeholder-row { background: #e5e7eb; color: #64748b; }
code { display: inline-block; background: #111827; color: #e5e7eb; padding: 8px; border-radius: 6px; }
"#;
