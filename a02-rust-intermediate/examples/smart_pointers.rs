use std::{cell::RefCell, rc::Rc};

fn main() {
    let shared_coupon = Rc::new(RefCell::new(DiscountCoupon {
        code: String::from("Summer Special"),
        remain_uses: 2,
    }));

    let order1 = Order {
        id: 1,
        item_name: "bag".to_string(),
        sub_order: None,
        coupon: Some(Rc::clone(&shared_coupon)),
    };
    let order2 = Order {
        id: 2,
        item_name: "bagpack".to_string(),
        sub_order: Some(Box::new(Order {
            id: 4,
            item_name: "zip".to_string(),
            sub_order: None,
            coupon: Some(Rc::clone(&shared_coupon)),
        })),
        coupon: Some(Rc::clone(&shared_coupon)),
    };

    println!("first order: {:?}", order1);
    println!("second order: {:?}", order2);

    // 첫번째 주문에서 coupon 한 개 사용하기
    if let Some(coupon_ref) = &order1.coupon {
        coupon_ref.borrow_mut().remain_uses -= 1;
    }

    if let Some(coupon_ref) = &order2.coupon {
        println!("남은 쿠폰: {}", coupon_ref.borrow().remain_uses);
    }

}

#[derive(Debug)]
struct Order {
    id: u32,
    item_name: String,
    sub_order: Option<Box<Order>>,
    coupon: Option<Rc<RefCell<DiscountCoupon>>>,
}

#[derive(Debug)]
struct DiscountCoupon {
    code: String,
    remain_uses: u32,
}
