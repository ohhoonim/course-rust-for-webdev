use uuid::Uuid;

#[derive(Debug, Clone)]
struct Order {
    id: Uuid,
    item_name: String,
    price: u32,
    quantity: u32,
    is_delivered: bool,

}
fn main() {

    let orders = vec![
        Order { id: Uuid::new_v4(), item_name: String::from("Keyboard"), price: 50000, quantity: 2, is_delivered: true },
        Order { id: Uuid::new_v4(), item_name: String::from("Mouse"), price: 30000, quantity: 1, is_delivered: false },
        Order { id: Uuid::new_v4(), item_name: String::from("Monitor"), price: 300000, quantity: 1, is_delivered: true },
        Order { id: Uuid::new_v4(), item_name: String::from("USB Cable"), price: 5000, quantity: 5, is_delivered: true },
        Order { id: Uuid::new_v4(), item_name: String::from("Headset"), price: 80000, quantity: 2, is_delivered: false },
    ];
    assert_eq!(complete_delivery(&orders).len(), 3, "필터 길이가 다릅니다.");
}

// 매개변수는 &[Order]와 같이 슬라이스로 표현해주는 것이 관례
fn complete_delivery(orders: &[Order]) -> Vec<&Order> {
    orders.iter()
        .filter(|order| order.is_delivered)
        .collect()
}

fn complete_delivery_into_iter(orders: Vec<Order>) -> Vec<Order> {
    orders
        .into_iter()
        .filter(|order| order.is_delivered)
        .collect()
}

fn complete_delivery_clone(orders: &[Order]) -> Vec<Order> {
    orders
        .iter()
        .filter(|order| order.is_delivered)
        .cloned() // 혹은 .map(|order| order.clone())
        .collect()
}

fn total_price(orders: &Vec<Order>) -> u32 {
    orders.iter()
        .map(|o| o.price * o.quantity)
        .sum()
}

fn expensive_items(orders: &[Order]) -> Vec<String> {
    orders.iter()
        .filter(|o| o.price * o.quantity > 100_000)
        .map(|o| o.item_name.clone())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_orders() -> Vec<Order> {
        vec![
            Order { id: Uuid::new_v4(), item_name: String::from("Keyboard"), price: 50000, quantity: 2, is_delivered: true },
            Order { id: Uuid::new_v4(), item_name: String::from("Mouse"), price: 30000, quantity: 1, is_delivered: false },
            Order { id: Uuid::new_v4(), item_name: String::from("Monitor"), price: 300000, quantity: 1, is_delivered: true },
            Order { id: Uuid::new_v4(), item_name: String::from("USB Cable"), price: 5000, quantity: 5, is_delivered: true },
            Order { id: Uuid::new_v4(), item_name: String::from("Headset"), price: 80000, quantity: 2, is_delivered: false },
        ]
    }
    #[test]
    fn test_complete_delivery() {
        let orders = make_orders();
        let filtered_orders = complete_delivery(&orders);
        assert_eq!(orders.len(), 5);
        assert_eq!(filtered_orders.len(), 3);
    }

    #[test]
    fn test_total_price() {
        let orders = make_orders();
        let total_price = total_price(&orders);
        assert_eq!(total_price, 615_000);
    }

    #[test]
    fn test_expensive_items() {
        let orders = make_orders();
        let expensive_items = expensive_items(&orders);
        println!("{:?}", expensive_items);
        assert_eq!(expensive_items.len(), 2);
    }
}