use leptos::prelude::*;
use leptos_router::{
    components::{ParentRoute, Route, Router, Routes},
    path,
};

mod components;
mod pages;
mod model;

use components::*;
use pages::*;


#[component]
pub fn App() -> impl IntoView {
    view! {
        <style>{STYLE}</style>
        <Router>
            <main>
                <h1>"Routing"</h1>
                <NavBar/>
                <Routes fallback=|| view! { <NotFoundPage/> }>
                    <Route path=path!("/") view=HomePage/>
                    <Route path=path!("/about") view=AboutPage/>
                    <Route path=path!("/contact") view=ContactPage/>
                    <Route path=path!("/users") view=UserListPage/>
                    <Route path=path!("/users/:id") view=UserDetailPage/>
                    <ParentRoute path=path!("/dashboard") view=DashboardLayout>
                        <Route path=path!("") view=DashboardHome/>
                        <Route path=path!("settings") view=DashboardSettings/>
                    </ParentRoute>
                </Routes>
            </main>
        </Router>
    }
}

const STYLE: &str = r#"
body { margin: 0; font-family: system-ui, sans-serif; background: #f8fafc; color: #17202a; }
main { max-width: 940px; margin: 0 auto; padding: 32px 16px; }
nav { display: flex; flex-wrap: wrap; gap: 8px; margin-bottom: 16px; }
a { color: #1f6feb; text-decoration: none; }
nav a, aside a { padding: 8px 10px; border-radius: 6px; background: #e5e7eb; color: #111827; }
a[aria-current="page"] { background: #1f6feb; color: white; }
.panel { background: white; border: 1px solid #d9dee8; border-radius: 8px; padding: 18px; margin: 14px 0; }
button, input { font: inherit; padding: 8px 10px; border-radius: 6px; border: 1px solid #b9c1d0; }
button { background: #1f6feb; color: white; cursor: pointer; border-color: #1f6feb; }
.cards { display: grid; gap: 8px; padding: 0; list-style: none; }
.cards li a { display: block; padding: 12px; border: 1px solid #e5e7eb; border-radius: 6px; }
.dashboard { display: grid; grid-template-columns: minmax(160px, 220px) 1fr; gap: 18px; }
aside { display: grid; align-content: start; gap: 8px; }
.dashboard-body { border-left: 1px solid #e5e7eb; padding-left: 18px; }
.error { border-color: #fecaca; background: #fff7f7; }
"#;
