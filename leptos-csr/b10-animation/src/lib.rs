mod components;
use leptos::prelude::*;
use components::*;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <style>{STYLE}</style>
        <main>
            <h1>"Animations"</h1>
            <FadeInOut/>
            <SlideDrawer/>
            <AnimatedCounter/>
            <StaggeredList/>
            <BouncingBall/>
        </main>
    }
}

const STYLE: &str = r#"
body { margin: 0; font-family: system-ui, sans-serif; background: #f8fafc; color: #17202a; }
main { max-width: 940px; margin: 0 auto; padding: 32px 16px; }
.panel { background: white; border: 1px solid #d9dee8; border-radius: 8px; padding: 18px; margin: 14px 0; position: relative; overflow: hidden; }
button { font: inherit; padding: 8px 10px; border-radius: 6px; border: 1px solid #1f6feb; background: #1f6feb; color: white; cursor: pointer; margin: 3px; }
.fade-box { margin-top: 12px; padding: 20px; border-radius: 8px; background: #eef2ff; color: #3730a3; }
.overlay { position: fixed; inset: 0; background: rgba(15,23,42,.36); transition: opacity .3s ease; z-index: 2; }
.drawer { position: fixed; top: 0; bottom: 0; left: 0; width: min(320px, 80vw); background: white; color: #17202a; padding: 24px; box-shadow: 12px 0 30px rgba(0,0,0,.18); transition: transform .3s ease; z-index: 3; }
.big-number { font-size: 3rem; font-weight: 800; margin: 8px 0; }
.staggered { display: grid; gap: 8px; padding: 0; list-style: none; }
.staggered li { padding: 10px; border-radius: 8px; background: #eef2ff; animation: slideIn .45s both; }
@keyframes slideIn { from { opacity: 0; transform: translateX(-20px); } to { opacity: 1; transform: translateX(0); } }
.stage { height: 220px; border: 1px solid #cbd5e1; border-radius: 8px; margin-top: 12px; position: relative; background: linear-gradient(#f8fafc, #e0f2fe); }
.ball { width: 44px; height: 44px; border-radius: 50%; background: #f97316; position: absolute; top: 0; left: 48%; box-shadow: inset -8px -8px 0 rgba(0,0,0,.12); }
"#;