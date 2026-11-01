use std::fmt;

// 1. 도메인 맞춤 커스텀 에러 정의
#[derive(Debug)]
enum ShopError {
    OutOfStock(String),
    InvalidCoupon(String),
    InsufficientFunds { required: u32, available: u32 },
}

impl std::error::Error for ShopError {}

impl fmt::Display for ShopError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ShopError::OutOfStock(item) => write!(f, "품절된 상품입니다: {}", item),
            ShopError::InvalidCoupon(code) => write!(f, "유효하지 않은 쿠폰 코드입니다: {}", code),
            ShopError::InsufficientFunds { required, available } => {
                write!(f, "잔액이 부족합니다. (필요: {}원, 보유: {}원)", required, available)
            }
        }
    }
}

// 2. 도메인 구조체 정의
struct Product {
    name: String,
    stock: u32,
    price: u32,
}

struct User {
    balance: u32,
}

// 3. 개별 검증 함수 (Result 반환)
fn check_stock(product: &Product, quantity: u32) -> Result<(), ShopError> {
    if product.stock < quantity {
        return Err(ShopError::OutOfStock(product.name.clone()));
    }
    Ok(())
}

fn apply_coupon(coupon_code: Option<&str>, price: u32) -> Result<u32, ShopError> {
    match coupon_code {
        Some("SUMMER_10") => Ok(price * 9 / 10), // 10% 할인
        Some(invalid) => Err(ShopError::InvalidCoupon(invalid.to_string())),
        None => Ok(price),
    }
}

// 4. ? 연산자를 이용한 에러 전파 및 결제 파이프라인 함수
fn checkout(
    product: &Product,
    quantity: u32,
    coupon_code: Option<&str>,
    user: &mut User,
) -> Result<u32, ShopError> {
    // 재고 검증 (에러 발생 시 즉시 반환)
    check_stock(product, quantity)?;

    // 기본 가격 계산 및 쿠폰 적용 (에러 발생 시 즉시 반환)
    let base_price = product.price * quantity;
    let final_price = apply_coupon(coupon_code, base_price)?;

    // 잔액 검증
    if user.balance < final_price {
        return Err(ShopError::InsufficientFunds {
            required: final_price,
            available: user.balance,
        });
    }

    // 결제 승인 (잔액 차감)
    user.balance -= final_price;
    Ok(final_price)
}

fn main() {
    let laptop = Product {
        name: String::from("MacBook Pro"),
        stock: 3,
        price: 1_500_000,
    };

    let mut user = User {
        balance: 2_000_000,
    };

    println!("--- 시나리오 1: 정상 결제 (쿠폰 적용) ---");
    match checkout(&laptop, 1, Some("SUMMER_10"), &mut user) {
        Ok(price) => println!("결제 완료! 최종 금액: {}원 (남은 잔액: {}원)", price, user.balance),
        Err(e) => println!("결제 실패: {}", e),
    }

    println!("\n--- 시나리오 2: 재고 초과 주문 ---");
    match checkout(&laptop, 5, None, &mut user) {
        Ok(price) => println!("결제 완료! 최종 금액: {}원", price),
        Err(e) => println!("결제 실패: {}", e),
    }

    println!("\n--- 시나리오 3: 잘못된 쿠폰 입력 ---");
    match checkout(&laptop, 1, Some("INVALID_CODE"), &mut user) {
        Ok(price) => println!("결제 완료! 최종 금액: {}원", price),
        Err(e) => println!("결제 실패: {}", e),
    }
}