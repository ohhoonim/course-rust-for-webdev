use leptos::prelude::*;

use crate::models::describe_error;

#[component]
pub fn ChartPanel() -> impl IntoView {
    let points = RwSignal::new(vec![12, 19, 8, 15, 22]);
    let initial_ready = chart_available();
    let chart_ready = RwSignal::new(initial_ready);
    let status = RwSignal::new(if initial_ready {
        "Chart.js ready".to_string()
    } else {
        "Loading Chart.js from script tag".to_string()
    });

    if !initial_ready {
        wait_for_chart_js(chart_ready, status, 0);
    }

    Effect::new(move |_| {
        let data = points.get();
        if chart_ready.get() {
            match render_chart(&data) {
                Ok(()) => status.set("Chart rendered from Rust data".into()),
                Err(message) => status.set(message),
            }
        }
    });

    on_cleanup(|| {
        let _ = js_sys::eval(
            "if (window.__interopChart) { window.__interopChart.destroy(); window.__interopChart = null; }",
        );
    });

    view! {
        <section class="panel">
            <h2>"Chart.js bridge"</h2>
            <button on:click=move |_| points.update(|items| items.rotate_left(1))>"Rotate data"</button>
            <div class="chart-wrap">
                <canvas id="interop-chart" width="480" height="180"></canvas>
            </div>
            <p role="status">{status}</p>
        </section>
    }
}

fn chart_available() -> bool {
    js_sys::eval("Boolean(window.Chart)")
        .ok()
        .and_then(|value| value.as_bool())
        .unwrap_or(false)
}

fn render_chart(points: &[i32]) -> Result<(), String> {
    match js_sys::eval(&chart_script(points)) {
        Ok(value) if value.as_bool() == Some(true) => Ok(()),
        Ok(_) => Err("Chart.js or the canvas was not found".into()),
        Err(error) => Err(format!("Chart.js failed ({})", describe_error(&error))),
    }
}

fn wait_for_chart_js(chart_ready: RwSignal<bool>, status: RwSignal<String>, attempt: u8) {
    if chart_available() {
        let _ = chart_ready.try_set(true);
        let _ = status.try_set("Chart.js ready".into());
        return;
    }
    if attempt >= 50 {
        let _ = status.try_set("Chart.js script did not load".into());
        return;
    }

    let timeout = gloo_timers::callback::Timeout::new(100, move || {
        wait_for_chart_js(chart_ready, status, attempt + 1);
    });
    timeout.forget();
}

fn chart_script(points: &[i32]) -> String {
    let data = points
        .iter()
        .map(|value| value.to_string())
        .collect::<Vec<_>>()
        .join(",");
    format!(
        r##"
(() => {{
    const canvas = document.getElementById("interop-chart");
    if (!window.Chart || !canvas) return false;
    if (window.__interopChart) {{
        window.__interopChart.destroy();
    }}
    window.__interopChart = new Chart(canvas, {{
        type: "bar",
        data: {{
            labels: ["A", "B", "C", "D", "E"],
            datasets: [{{
                label: "Rust data",
                data: [{data}],
                backgroundColor: "#1f6feb"
            }}]
        }},
        options: {{
            responsive: true,
            maintainAspectRatio: false,
            animation: false
        }}
    }});
    return true;
}})()
"##
    )
}

#[cfg(test)]
mod tests {
    use super::*;

     #[test]
    fn chart_script_carries_only_numbers() {
        let script = chart_script(&[12, -3, 0]);
        assert!(script.contains("data: [12,-3,0]"));
        assert!(script.contains("return true;"));
        assert!(script.contains("return false;"));
    }

    #[test]
    fn chart_script_handles_no_data() {
        assert!(chart_script(&[]).contains("data: []"));
    }
}