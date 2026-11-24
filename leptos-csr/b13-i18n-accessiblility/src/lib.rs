use leptos::prelude::*;

mod models;
mod components;
use components::*;
use models::*;

#[component]
pub fn App() -> impl IntoView {
    let lang = RwSignal::new(initial_language());
    provide_context(lang);

    Effect::new(move |_| {
        let current = lang.get();
        if let Some(root) = document().document_element() {
            let _ = root.set_attribute("lang", current.code());
            let _ = root.set_attribute("dir", current.dir());
        }
        document().set_title(t(current, "title"));
    });

    view! {
        <style>{STYLE}</style>
        <main>
            <h1>"i18n and Accessibility"</h1>
            <LanguageSwitcher/>
            <GreetingCard/>
            <CheckoutButton/>
            <AlertBanner/>
            <AccessibleForm/>
            <DirectionPreview/>
            <KeyboardMenu/>
        </main>
    }
}

const STYLE: &str = r#"
body { margin: 0; font-family: system-ui, sans-serif; background: #f8fafc; color: #17202a; }
main { max-width: 900px; margin: 0 auto; padding: 32px 16px; }
.panel { background: white; border: 1px solid #d9dee8; border-radius: 8px; padding: 18px; margin: 14px 0; }
.row, .menu { display: flex; gap: 10px; align-items: center; flex-wrap: wrap; }
.stack { display: grid; gap: 8px; max-width: 360px; }
button, input { font: inherit; padding: 8px 10px; border-radius: 6px; border: 1px solid #94a3b8; }
button { background: #e5e7eb; cursor: pointer; }
button.active { background: #1f6feb; color: white; border-color: #1f6feb; }
.alert { padding: 12px; border-radius: 8px; background: #ecfdf5; color: #065f46; font-weight: 700; }
.direction-card { border: 1px dashed #94a3b8; border-radius: 8px; padding: 14px; }
.alert-region { min-height: 3.2em; margin-top: 10px; }
button:focus-visible, input:focus-visible { outline: 3px solid #1f6feb; outline-offset: 2px; }
"#;
