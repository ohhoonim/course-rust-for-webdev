
#[derive(Debug)]
struct Order {
    item_name: String,
}

fn main() {
    let order1 = Order {
        item_name: String::from("bag"),
    };
    let order2 = Order {
        item_name: String::from("backpack"),
    };

    let featured = FeatureOrder {
        order_item: get_longer_order(&order1, &order2),
    };
    println!("{:?}", featured.order_item);
}

fn get_longer_order<'a>(order1: &'a Order, order2: &'a Order) -> &'a Order {
    if order1.item_name.len() > order2.item_name.len() {
        order1
    } else {
        order2
    }
}

struct FeatureOrder<'a> {
    order_item: &'a Order,
}