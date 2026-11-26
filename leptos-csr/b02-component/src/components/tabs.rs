use leptos::prelude::*;

#[slot]
pub struct Tab {
    pub name: String,
    #[prop(optional)]
    pub children: Option<ChildrenFn>,
}

#[component]
pub fn Tabs(
    #[prop(optional)]
    tabs: Vec<Tab>,
) -> impl IntoView {
    let (selected_index, set_selected_index) = signal(0);
    let tabs = StoredValue::new(tabs);

    view! {
        <div class="tabs-container">
            <div class="tab-buttons">
                {tabs.with_value(|tabs| {
                    tabs.iter()
                        .enumerate()
                        .map(|(index, tab)| {
                            let name = tab.name.clone();
                            view! {
                                <button
                                    on:click=move |_| set_selected_index.set(index)
                                    class=move || if selected_index.get() == index { "active" } else { "" }
                                >
                                    {name}
                                </button>
                            }
                        })
                        .collect::<Vec<_>>()
                })}
            </div>

            <div class="tab-content">
                {move || {
                    let idx = selected_index.get();
                    tabs.with_value(|tabs| {
                        tabs.get(idx)
                            .and_then(|tab| tab.children.as_ref())
                            .map(|children| children().into_any())
                            .unwrap_or_else(|| ().into_any())
                    })
                }}
            </div>
        </div>
    }
}