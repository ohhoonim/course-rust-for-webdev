use leptos::{html, prelude::*};
use wasm_bindgen::prelude::{Closure, JsCast};

#[component]
pub fn IntersectionPanel() -> impl IntoView {
    let target_ref: NodeRef<html::Div> = NodeRef::new();
    let visible = use_intersection(target_ref);

    view! {
        <section class="panel">
            <h2>"IntersectionObserver"</h2>
            <p>"Scroll down: the box below reports when it is on the screen."</p>
            <div class="spacer" aria-hidden="true"></div>
            <div node_ref=target_ref class="target" role="status">
                {move || if visible.get() { "Visible" } else { "Waiting for the observer" }}
            </div>
        </section>
    }
}

struct ObserverHandle {
    observer: web_sys::IntersectionObserver,
    _callback: Closure<dyn FnMut(js_sys::Array)>,
}

fn use_intersection(node: NodeRef<html::Div>) -> ReadSignal<bool> {
    let (visible, set_visible) = signal(false);
    let handle: StoredValue<Option<ObserverHandle>, LocalStorage> = StoredValue::new_local(None);

    Effect::new(move |_| {
        let Some(element) = node.get() else {
            return;
        };
        if handle.with_value(|existing| existing.is_some()) {
            return;
        }
        let callback = Closure::<dyn FnMut(js_sys::Array)>::new(move |entries: js_sys::Array| {
            // The last entry is the most recent state of the element.
            if let Ok(entry) = entries
                .pop()
                .dyn_into::<web_sys::IntersectionObserverEntry>()
            {
                let _ = set_visible.try_set(entry.is_intersecting());
            }
        });
        if let Ok(observer) = web_sys::IntersectionObserver::new(callback.as_ref().unchecked_ref())
        {
            observer.observe(&element);
            handle.set_value(Some(ObserverHandle {
                observer,
                _callback: callback,
            }));
        }
    });

    on_cleanup(move || {
        if let Some(Some(handle)) = handle.try_update_value(|value| value.take()) {
            handle.observer.disconnect();
        }
    });

    visible
}