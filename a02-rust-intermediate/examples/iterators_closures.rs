fn main() {}

fn init_orders() -> Vec<Order> {
    vec![
        Order {
            id: 1,
            item_name: String::from("Keyboard"),
            price: 50000,
            quantity: 2,
            is_delivered: true,
        },
        Order {
            id: 2,
            item_name: String::from("Mouse"),
            price: 30000,
            quantity: 1,
            is_delivered: false,
        },
        Order {
            id: 3,
            item_name: String::from("Monitor"),
            price: 300000,
            quantity: 1,
            is_delivered: true,
        },
        Order {
            id: 4,
            item_name: String::from("USB Cable"),
            price: 5000,
            quantity: 5,
            is_delivered: true,
        },
        Order {
            id: 5,
            item_name: String::from("Headset"),
            price: 80000,
            quantity: 2,
            is_delivered: false,
        },
    ]
}

#[derive(Debug)]
struct Order {
    id: u32,
    item_name: String,
    price: u32,
    quantity: u32,
    is_delivered: bool,
}
// 배송 완료된 주문만 필터링 (filter): is_delivered가 true인 주문들만 골라냅니다.
fn filter_delivered(orders: &[Order]) -> Vec<&Order> {
    orders.iter().filter(|order| order.is_delivered).collect()
}
// 총 주문 금액 계산 (map과 fold 또는 sum): 필터링된 각 주문의 가격 곱하기 수량(price * quantity)을 계산한 뒤, 총합을 구합니다.
fn filter_delivered_sum(orders: &[Order]) -> u32 {
    orders
        .iter()
        .filter(|order| order.is_delivered)
        .map(|order| order.price * order.quantity)
        .sum()
}

// 배송 여부와 상관없이 총 가격(price * quantity)이 100,000원을 초과하는 상품들의 이름(item_name)만 모아서 새로운 Vec<String>으로 수집합니다.
fn over_hundred_thousand(orders: &[Order]) -> Vec<String> {
    orders
        .iter()
        .filter(|order| order.price * order.quantity > 100_000)
        .map(|order| order.item_name.clone())
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn over_hundred() {
        let result = over_hundred_thousand(&init_orders());
        assert_eq!(result.len(), 2);
    }

    #[test]
    fn fiter_test() {
        let orders = init_orders();
        let result = filter_delivered(&orders);

        assert_eq!(result.len(), 3);
    }

    #[test]
    fn order_total_test() {
        let total = filter_delivered_sum(&init_orders());
        assert_eq!(total, 425000);
    }

    // 과제 1: iter_mut()를 이용한 일괄 할인 적용
    #[test]
    fn homework_1() {
        // iter_mut를 사용하려면 mut 로 변수선언을 해야함.
        let mut orders = init_orders();

        println!("변경전: {:?}", orders);

        orders
            .iter_mut()
            .filter(|order| !order.is_delivered)
            .for_each(|order| {
                order.price = order.price * 90 / 100;
            });

        println!("변경후: {:?}", orders);
    }

    #[derive(Debug)]
    struct OrderSummary {
        id: u32,
        description: String, // 예: "Keyboard (2개)" 형태로 포맷팅
    }

    //기존의 orders 벡터를 .into_iter()로 소비하면서, 각 주문을 OrderSummary로 변환하는 코드를 작성하세요.
    #[test]
    fn homework_2() {
        let orders = init_orders();

        let order_summary: Vec<OrderSummary> = orders.into_iter()
            .map(|order| OrderSummary {
                id: order.id,
                description: format!("{} ({}개)", order.item_name, order.quantity)
            }).collect();

        println!("{:?}", order_summary);

        // println!("{:?}", orders); // 에러남. into_iter() 로 이미 이전되었으므로

    }
}
