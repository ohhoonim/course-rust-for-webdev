
# Error Handling️

## 목표

- 회복 가능성과 불가능한 에러의 분리: 프로그램이 중단되어야 하는 상황(`panic!`)과 복구하여 안전하게 처리해야 하는 상황(`Result<T, E>`)을 구분한다.
- `Result`와 `Option` 활용: 에러나 값이 존재하지 않을 수 있는 상황을 안전하게 다루기 위해 `match`, `unwrap`, `expect` 및 조합자 메서드를 사용한다.
- `?` 연산자를 이용한 에러 전파: 반복적인 `match` 문을 줄이고, 에러를 상위 호출자로 깔끔하게 위임한다.
- 커스텀 에러 정의: 도메인에 맞는 명확한 에러 열거형(`enum Error`)을 정의하고 표준 에러 트레이트와 연동한다.

## 상황: 온라인 쇼핑몰 주문 및 결제 검증 시스템

주문(Order)을 생성하고 결제를 진행하는 과정에서 발생할 수 있는 다양한 예외 상황을 안전하게 처리하는 시스템을 구현합니다.

- 상황 1 (재고 검증 - `Option` 활용): 장바구니에 담은 상품의 재고가 존재하지 않거나 품절된 경우, 데이터가 없음을 나타내는 `Option`을 활용해 예외를 처리합니다.
- 상황 2 (쿠폰 및 가격 검증 - `Result` 활용): 만료된 쿠폰을 적용하거나 보유 잔액보다 결제 금액이 큰 경우, 구체적인 에러 사유를 담은 `Result`를 반환합니다.
- 상황 3 (에러 전파 - `?` 연산자 활용): 재고 확인 -> 쿠폰 검증 -> 결제 승인으로 이어진 복잡한 단계 중 어디에서든 에러가 발생하면 즉시 상위 함수로 에러를 전파하여 안전하게 거래를 중단시킵니다.

## 힌트

- 도메인 구조체
    
    ```rust
    struct Product {
        name: String,
        stock: u32,
        price: u32,
    }
    
    struct User {
        balance: u32,
    }
    ```
    
- ? 연산자를 이용한 에러 전파
    
    러스트에서 `?` 연산자는 `Result`나 `Option` 타입을 반환하는 함수에서 에러나 값이 없을 때의 처리를 매우 간결하게 만들어 주는 에러 전파(Error Propagation) 문법 설탕(Syntactic Sugar)이다.
    
    장황한 `match` 문을 작성하는 대신, 에러가 발생했을 때 즉시 상위 호출자로 에러를 반환하는 코드를 한 글자로 압축한다.
    
    - 동작 방식
        
        `?` 연산자가 붙은 표현식의 결과는 다음과 같이 동작한다.
        
        1. 성공인 경우 (`Ok(T)` 또는 `Some(T)`):
            - 에러나 None이 아니라면, 감싸져 있던 내부 값(`T`)을 꺼내어 변수에 대입하거나 그대로 진행한다.
        2. 실패인 경우 (`Err(E)` 또는 `None`):
            - 현재 함수의 실행을 즉시 중단하고, 해당 에러(`Err(E)`)를 현재 함수의 반환값으로 곧바로 리턴한다.
    - 코드 비교: 없이 쓰기 vs `?` 연산자 사용
        - 1. `?` 연산자가 없을 때 (`match` 문 사용)
        
        ```rust
        fn parse_number(s: &str) -> Result<i32, std::num::ParseIntError> {
            let parsed = match s.parse::<i32>() {
                Ok(n) => n,
                Err(e) => return Err(e), // 에러를 수동으로 반환
            };
            Ok(parsed * 2)
        }
        ```
        
        - 2. `?` 연산자를 사용할 때
        
        ```rust
        fn parse_number(s: &str) -> Result<i32, std::num::ParseIntError> {
            let parsed = s.parse::<i32>()?; // 에러가 나면 알아서 Err(e)를 리턴함
            Ok(parsed * 2)
        }
        ```
        
    - 주의할 점: 에러 타입의 호환성
        
        `?` 연산자는 단순히 `return Err(e);`를 대신해 주는 것이 아니라, 반환하려는 에러 타입으로 자동 변환(`From` 트레이트)을 시도합니다.
        
        따라서 `?`를 사용하는 함수의 반환 타입(`Result<T, E_func>`)이 내부에서 호출하는 함수의 에러 타입(`E_inner`)을 수용할 수 있거나, `From` 구현을 통해 변환이 가능한 상태여야 컴파일 에러가 나지 않는다.
        
- panic!
    
    러스트에서 `panic!`은 프로그램이 더 이상 정상적으로 실행될 수 없는 복구 불가능한 에러(Unrecoverable Error)가 발생했을 때 사용한다.
    
    `panic!`이 호출되면 프로그램은 즉시 에러 메시지와 스택 추적(Backtrace) 정보를 출력하고 안전하게(또는 즉시) 종료한다.  `Result`가 "에러를 처리(복구)할 수 있는 경우"에 사용된다면, `panic!`은 "절대 일어나서는 안 되는 버그나 치명적인 상태"를 마주했을 때 발생시킨다.
    
    - 예제 코드
    
    ```rust
    fn check_age(age: i32) {
        if age < 0 {
            // 프로그램이 유지될 수 없는 치명적인 상태이므로 패닉 발생
            panic!("나이는 음수가 될 수 없습니다! 입력된 값: {}", age);
        }
        println!("나이 확인 완료: {}세", age);
    }
    
    fn main() {
        check_age(25); // 정상 작동
    
        // 이 코드가 실행되면 패닉이 발생하며 프로그램이 즉시 중단된다.
        check_age(-5);
    }
    ```
    

## 결과

```rust
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
```

## 결과(thiserror 라이브러리 활용)

```rust
use thiserror::Error;

// thiserror를 사용한 간결하고 직관적인 커스텀 에러 정의
#[derive(Error, Debug)]
pub enum ShopError {
    #[error("품절된 상품입니다: {0}")]
    OutOfStock(String),

    #[error("유효하지 않은 쿠폰 코드입니다: {0}")]
    InvalidCoupon(String),

    #[error("잔액이 부족합니다. (필요: {required}원, 보유: {available}원)")]
    InsufficientFunds { required: u32, available: u32 },

    // 외부 에러(std::io::Error)를 자동으로 변환(From 구현)해 주는 어트리뷰트
    #[error("시스템 내부 I/O 오류가 발생했습니다.")]
    IoError(#[from] std::io::Error),
}

struct Product {
    name: String,
    stock: u32,
    price: u32,
}

struct User {
    balance: u32,
}

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

fn checkout(
    product: &Product,
    quantity: u32,
    coupon_code: Option<&str>,
    user: &mut User,
) -> Result<u32, ShopError> {
    check_stock(product, quantity)?;

    let base_price = product.price * quantity;
    let final_price = apply_coupon(coupon_code, base_price)?;

    if user.balance < final_price {
        return Err(ShopError::InsufficientFunds {
            required: final_price,
            available: user.balance,
        });
    }

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

    println!("--- 정상 결제 테스트 ---");
    match checkout(&laptop, 1, Some("SUMMER_10"), &mut user) {
        Ok(price) => println!("결제 성공! 최종 금액: {}원 (남은 잔액: {}원)", price, user.balance),
        Err(e) => println!("결제 실패: {}", e),
    }

    println!("\n--- 잘못된 쿠폰 테스트 ---");
    match checkout(&laptop, 1, Some("BAD_COUPON"), &mut user) {
        Ok(price) => println!("결제 성공! 최종 금액: {}원", price),
        Err(e) => println!("결제 실패: {}", e),
    }
}
```
