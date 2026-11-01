fn main() {}

//1. 공통 인터페이스 정의 및 다형성 (Shared Interface & Polymorphism)

trait Describable {
    fn describe(&self) -> String;
}

struct Order {
    item_name: String,
    price: u32,
}

struct Membership {
    tier: String,
}

impl Describable for Order {
    fn describe(&self) -> String {
        format!("주문 상품: {}, 가격: {}원", self.item_name, self.price)
    }
}

impl Describable for Membership {
    fn describe(&self) -> String {
        format!("멤버십 등급: {}", self.tier)
    }
}

fn print_info<T: Describable>(item: &T) {
    println!("{}", item.describe());
}

// 2. 확장 트레이트 (Extension Traits)
// 표준 타입(str)이나 외부 라이브러리 타입을 직접 수정할 수 없을 때

// 문자열에 주문 번호 포맷팅 기능을 추가
trait OrderIdExt {
    fn to_order_tag(&self) -> String;
}

impl OrderIdExt for str {
    fn to_order_tag(&self) -> String {
        format!("[ORDER-ID]: {}", self)
    }
}

// 3. 연산자 오버로딩 및 표준 동작 커스터마이징 (Operator Overloading & Standard Integration)
use std::ops::Add;

#[derive(Debug)]
struct Money(f32);

impl Add for Money {
    type Output = Money;

    fn add(self, rhs: Self) -> Self::Output {
        Money(self.0 + rhs.0)
    }
}

// ///////////////////////////////////////////

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn shared_interface_polymorphism() {
        let order = Order {
            item_name: String::from("bag"),
            price: 100_000,
        };
        let membership = Membership {
            tier: String::from("Silver"),
        };

        print_info(&order);
        print_info(&membership);
    }

    #[test]
    fn str_ext_to_order_id() {
        let raw_id = "A1B1C-2";
        println!("{}", raw_id.to_order_tag());
    }

    #[test]
    fn override_add_for_money() {
        let m1 = Money(100_000.12);
        let m2 = Money(1_900.10);

        let total = m1 + m2;
        println!("{:2}", total.0);
    }
}
