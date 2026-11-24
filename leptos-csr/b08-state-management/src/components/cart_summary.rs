use leptos::prelude::*;
use crate::models::{Level, use_notify};

#[derive(Clone)]
pub struct CartItem {
    id: u32,
    name: String,
    price: f64,
    qty: u32,
}

#[component]
pub fn CartSummary() -> impl IntoView {
    let cart = use_context::<RwSignal<Vec<CartItem>>>().expect("cart context");
    let count = Memo::new(move |_| cart.read().iter().map(|item| item.qty).sum::<u32>());
    let total = Memo::new(move |_| {
        cart.read()
            .iter()
            .map(|item| item.price * item.qty as f64)
            .sum::<f64>()
    });

    view! {
        <section class="panel">
            <h2>"Shopping Cart Context"</h2>
            <div class="grid">
                <ProductCard id=1 name="Leptos Hoodie" price=64.0/>
                <ProductCard id=2 name="Rust Mug" price=18.5/>
                <ProductCard id=3 name="WASM Sticker Pack" price=9.0/>
            </div>
            <p><strong>{count}</strong> " items, total " <strong>{move || format!("${:.2}", total.get())}</strong></p>
            <ul>
                <For
                    each=move || cart.get()
                    key=|item| item.id
                    children=move |item| {
                        let id = item.id;
                        let qty = move || {
                            cart.read().iter().find(|line| line.id == id).map(|line| line.qty).unwrap_or(0)
                        };
                        view! { <li>{item.name} " x" {qty}</li> }
                    }
                />
            </ul>
        </section>
    }
}


#[component]
fn ProductCard(id: u32, #[prop(into)] name: String, price: f64) -> impl IntoView {
    let cart = use_context::<RwSignal<Vec<CartItem>>>().expect("cart context");
    let notify = use_notify();
    let name_for_click = name.clone();

    view! {
        <article class="card">
            <strong>{name}</strong>
            <p>{format!("${price:.2}")}</p>
            <button on:click=move |_| {
                cart.update(|items| {
                    if let Some(existing) = items.iter_mut().find(|item| item.id == id) {
                        existing.qty += 1;
                    } else {
                        items.push(CartItem { id, name: name_for_click.clone(), price, qty: 1 });
                    }
                });
                notify(format!("Added {name_for_click}"), Level::Success);
            }>"Add to Cart"</button>
        </article>
    }
}
