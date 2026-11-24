use leptos::prelude::*;

mod components;

use components::*;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <style>{STYLE}</style>
        <main>
            <h1>"Styling"</h1>
            <TabBar/>
            <MotionCard/>
            <GradientPreview/>
            <ThemePalette/>
            <DarkModeToggle/>
        </main>
    }
}

const STYLE: &str = r#"
:root { --primary: #2563eb; }
.dark body { background: #111827; color: #e5e7eb; }
body { margin: 0; font-family: system-ui, sans-serif; background: #f8fafc; color: #17202a; transition: background .2s ease, color .2s ease; }
main { max-width: 940px; margin: 0 auto; padding: 32px 16px; }
.panel { background: color-mix(in srgb, currentColor 4%, transparent); border: 1px solid #94a3b8; border-radius: 8px; padding: 18px; margin: 14px 0; }
.row, .tabs, .swatches { display: flex; flex-wrap: wrap; gap: 10px; align-items: center; }
input, button { font: inherit; padding: 8px 10px; border-radius: 6px; border: 1px solid #94a3b8; }
button { background: #e5e7eb; color: #111827; cursor: pointer; }
button.active, .tabs button.active { background: var(--primary); color: white; border-color: var(--primary); }
.motion-card { margin-top: 14px; padding: 18px; border: 1px solid currentColor; border-radius: 8px; transition: opacity .2s ease, transform .2s ease, color .2s ease; }
.gradient { min-height: 120px; display: grid; place-items: center; margin-top: 12px; color: white; font-weight: 700; border-radius: 8px; text-shadow: 0 1px 2px rgba(0,0,0,.45); }
.swatches button { color: white; border-color: transparent; min-width: 86px; text-shadow: 0 1px 2px rgba(0,0,0,.45); }
.brand { color: var(--primary); font-weight: 800; }
"#;