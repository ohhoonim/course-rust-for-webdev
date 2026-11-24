mod components;
mod models;

use leptos::prelude::*;
use components::*;
use models::*;


#[component]
pub fn App() -> impl IntoView {
    let theme = RwSignal::new(Theme::Light);
    provide_context(theme);
    provide_context(RwSignal::new(Vec::<CartItem>::new()));
    provide_context(RwSignal::new(Vec::<Notification>::new()));
    provide_context(NextNotificationId(RwSignal::new(1)));

    view! {
        <style>{STYLE}</style>
        <div class=move || if theme.get() == Theme::Light { "app light" } else { "app dark" }>
            <main>
                <h1>"상태 관리"</h1>
                <section class="panel row between">
                    <span>"Theme context is provided at the root."</span>
                    <ThemeToggle/>
                </section>
                <CartSummary/>
                <Wizard/>
                <PersistentCounter/>
                <NotificationDemo/>
                <NotificationToast/>
            </main>
        </div>
    }
}

const STYLE: &str = r#"
body { margin: 0; font-family: system-ui, sans-serif; }
.app { min-height: 100vh; padding: 1px 0; }
.light { background: #f8fafc; color: #17202a; }
.dark { background: #111827; color: #e5e7eb; }
main { max-width: 940px; margin: 0 auto; padding: 32px 16px; }
.panel { background: color-mix(in srgb, currentColor 4%, transparent); border: 1px solid #94a3b8; border-radius: 8px; padding: 18px; margin: 14px 0; }
.grid { display: grid; grid-template-columns: repeat(auto-fit, minmax(180px, 1fr)); gap: 12px; }
.card { border: 1px solid #cbd5e1; border-radius: 8px; padding: 12px; }
.stack { display: grid; gap: 10px; max-width: 430px; }
.row { display: flex; gap: 10px; align-items: center; flex-wrap: wrap; }
.between { justify-content: space-between; }
input, button { font: inherit; padding: 8px 10px; border-radius: 6px; border: 1px solid #94a3b8; }
button { background: #1f6feb; color: white; cursor: pointer; border-color: #1f6feb; margin: 3px; }
.progress { height: 10px; background: #cbd5e1; border-radius: 999px; overflow: hidden; margin-bottom: 12px; }
.progress div { height: 100%; background: #1f6feb; transition: width 0.2s ease; }
.success { color: #047857; font-weight: 700; }
.toasts { position: fixed; right: 16px; bottom: 16px; display: grid; gap: 8px; }
.toast { padding: 10px 12px; border-radius: 8px; color: white; box-shadow: 0 8px 30px rgba(0,0,0,.18); }
.toast.info { background: #2563eb; }
.toast.success { background: #047857; }
.toast.warning { background: #b45309; }
"#;