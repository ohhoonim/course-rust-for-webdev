
# Advanced Traits️

## 목표

트레이트(Trait)는 단순히 인터페이스 개념을 넘어 3가지 용법에 대해 이해하고 활용한다.

- 공통 인터페이스 정의 및 다형성 (Shared Interface & Polymorphism)
    - 제네릭의 트레이트 바운드(`T: Summary`)를 통해 컴파일 타임에 안전한 정적 다형성을 구현하거나, 트레이트 객체(`dyn Trait`)를 통해 런타임 동적 다형성을 구현할 수 있다.
- 확장 트레이트 (Extension Traits)
    - 특정 타입에 존재하지 않는 도메인 고유의 편의 메서드를 추가하여 코드의 가독성을 높이고 체이닝(Chaining) 패턴을 구현할 때 주로 사용
- 연산자 오버로딩 및 표준 동작 커스터마이징 (Operator Overloading & Standard Trait Integration)
    - `Add`, `Sub` 등의 트레이트를 구현하여 커스텀 타입끼리 수학 연산자(`+`, )를 사용할 수 있게 한다 (`연산자 오버로딩`).
    - `Display`, `Debug`를 구현해 `println!` 출력을 지원
    - `Drop`을 구현해 객체가 소멸할 때의 커스텀 정리 로직을 실행

## 상황

- 1. 공통 인터페이스 정의 및 다형성
    
    Order, Membership struct에 ‘describe()’ 공통 메서드를 정의한다.
    
- 2. 확장 트레이트
    
    문자열 슬라이스(&str)에 주문 번호 포맷팅(’[ORDER-ID]: {}’) 기능을 추가한다.
    
- 3. 연산자 오버로딩 및 표준 동작 커스터마이징
    
    Money struct를 만들고, std::ops::Add 트레이트를 오버라이드하여 연산자 ‘+’ 기능을 추가한다.
    

## 힌트

```rust
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
```

```rust
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
```

```rust

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
```

## 결과

- 1. 공통 인터페이스 정의 및 다형성

```rust
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
```

- 2. 확장 트레이트

```rust
fn str_ext_to_order_id() {
    let raw_id = "A1B1C-2";
    println!("{}", raw_id.to_order_tag());
}
```

- 3. 연산자 오버로딩 및 표준 동작 커스터마이징

```rust
fn override_add_for_money() {
    let m1 = Money(100_000.12);
    let m2 = Money(1_900.10);

    let total = m1 + m2;
    println!("{:2}", total.0);
}
```

## 과제

트레이트의 3가지 용법을 직접 구현하고 체득할 수 있는 이커머스 도메인 기반의 실습 과제입니다.

- 과제 1: 공통 인터페이스와 다형성 (`Summary` 트레이트)
    - 목표: 서로 다른 상품 타입에 공통 인터페이스를 부여하고, 제네릭 함수를 통해 다형적으로 처리하기.
    - 요구사항:
        1. `PhysicalProduct`(실물 상품)와 `DigitalProduct`(디지털 상품) 구조체를 각각 정의하세요. (필드는 자율 구성, 예: 이름, 가격, 용량 등)
        2. 두 구조체가 공통으로 구현해야 하는 `Summary` 트레이트를 정의하세요. 이 트레이트는 상품의 요약 정보를 `String`으로 반환하는 `summarize(&self) -> String` 메서드를 가져야 합니다.
        3. 각 구조체에 `Summary` 트레이트를 구현하세요.
        4. `T: Summary` 제네릭 바운드를 사용하는 함수 `print_product_summary(item: &T)`를 작성하고, `main` 함수에서 두 상품을 인자로 전달해 각각 출력해 보세요.
- 과제 2: 확장 트레이트 (`PriceFormatter` 확장)
    - 목표: 기본 원시 타입(`u32` 등)에 도메인 전용 포맷팅 메서드 부착하기.
    - 요구사항:
        1. 숫자(`u32`) 타입에 원화(KRW) 형식의 문자열로 변환해 주는 메서드 `.to_krw()`를 추가하는 확장 트레이트 `PriceExt`를 정의하세요.
        2. `u32` 타입에 `PriceExt`를 구현하여, 숫자가 주어졌을 때 천 단위 콤마와 `"원"`이 붙은 문자열을 반환하도록 만드세요 (예: `15000` -> `"15,000원"`).
        3. `main` 함수에서 일반 정수(`let price = 50000;`)에 `.to_krw()` 메서드를 직접 호출하여 결과를 출력해 보세요.
- 과제 3: 연산자 오버로딩 (`Cart` 포인트 합산)
    - 목표: 표준 라이브러리의 연산자 트레이트를 구현하여 커스텀 타입 간 연산 지원하기.
    - 요구사항:
        1. 적립 포인트를 표현하는 구조체 `RewardPoints`를 정의하세요 (`points: u32`).
        2. 표준 라이브러리의 `std::ops::Add` 트레이트를 `RewardPoints`에 구현하여, 두 포인트 객체끼리 `+` 연산자를 사용해 합산할 수 있도록 만드세요.
        3. 디버그 출력을 위해 `#[derive(Debug, PartialEq)]`를 적용하세요.
        4. `main` 함수에서 두 개의 `RewardPoints` 인스턴스를 더한 결과가 정상적으로 계산되는지 확인하고 출력해 보세요.

## 과제 풀이

```rust
use std::ops::Add;

// --- 과제 1: 공통 인터페이스와 다형성 ---
trait Summary {
    fn summarize(&self) -> String;
}

#[derive(Debug)]
struct PhysicalProduct {
    name: String,
    price: u32,
    weight: f64,
}

#[derive(Debug)]
struct DigitalProduct {
    name: String,
    price: u32,
    file_size_mb: u32,
}

impl Summary for PhysicalProduct {
    fn summarize(&self) -> String {
        format!("[실물 상품] {} (가격: {}원, 무게: {}kg)", self.name, self.price, self.weight)
    }
}

impl Summary for DigitalProduct {
    fn summarize(&self) -> String {
        format!("[디지털 상품] {} (가격: {}원, 용량: {}MB)", self.name, self.price, self.file_size_mb)
    }
}

fn print_product_summary<T: Summary>(item: &T) {
    println!("{}", item.summarize());
}

// --- 과제 2: 확장 트레이트 ---
trait PriceExt {
    fn to_krw(&self) -> String;
}

impl PriceExt for u32 {
    fn to_krw(&self) -> String {
        let s = self.to_string();
        let mut result = String::new();
        let len = s.len();
        for (i, c) in s.chars().enumerate() {
            if i > 0 && (len - i) % 3 == 0 {
                result.push(',');
            }
            result.push(c);
        }
        result.push_str("원");
        result
    }
}

// --- 과제 3: 연산자 오버로딩 ---
#[derive(Debug, PartialEq, Clone, Copy)]
struct RewardPoints {
    points: u32,
}

impl Add for RewardPoints {
    type Output = RewardPoints;

    fn add(self, other: RewardPoints) -> RewardPoints {
        RewardPoints {
            points: self.points + other.points,
        }
    }
}

fn main() {
    // 과제 1 실행
    let physical = PhysicalProduct {
        name: String::from("Mechanical Keyboard"),
        price: 89_000,
        weight: 1.2,
    };
    let digital = DigitalProduct {
        name: String::from("Rust Programming E-Book"),
        price: 25_000,
        file_size_mb: 15,
    };

    print_product_summary(&physical);
    print_product_summary(&digital);

    // 과제 2 실행
    let price: u32 = 150000;
    println!("포맷된 가격: {}", price.to_krw());

    // 과제 3 실행
    let p1 = RewardPoints { points: 1500 };
    let p2 = RewardPoints { points: 3000 };
    let total_points = p1 + p2;
    println!("합산된 포인트: {:?}", total_points);
}
```
