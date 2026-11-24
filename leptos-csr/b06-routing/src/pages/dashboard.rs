use leptos::prelude::*;
use leptos_router::components::{A, Outlet};

#[component]
pub fn DashboardLayout() -> impl IntoView {
    view! {
        <section class="panel dashboard">
            <aside>
                <h2>"Dashboard"</h2>
                <A href="/dashboard" exact=true>"Overview"</A>
                <A href="/dashboard/settings">"Settings"</A>
            </aside>
            <div class="dashboard-body">
                <Outlet/>
            </div>
        </section>
    }
}

#[component]
pub fn DashboardHome() -> impl IntoView {
    view! {
        <div>
            <h3>"Dashboard Home"</h3>
            <p>"Nested index route rendered inside the dashboard layout."</p>
        </div>
    }
}

#[component]
pub fn DashboardSettings() -> impl IntoView {
    let email_reports = RwSignal::new(true);
    view! {
        <div>
            <h3>"Dashboard Settings"</h3>
            <label>
                <input
                    type="checkbox"
                    prop:checked=email_reports
                    on:change:target=move |event| email_reports.set(event.target().checked())
                />
                " Email weekly routing reports"
            </label>
        </div>
    }
}