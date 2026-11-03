
# Lifetimes

## 목표

- 댕글링 참조(Dangling Reference) 방지 원리 이해: 러스트 컴파일러가 메모리 안정성을 보장하기 위해 참조의 유효 기간을 어떻게 추적하는지 파악
- 명시적 수명 주석(`'a`) 문법 : 함수나 구조체에서 여러 참조자가 얽혀 있을 때 컴파일러에게 관계를 명시해 주는 표기법을 사용한다.
- 수명 생략 규칙(Lifetime Elision Rules) 이해: 개발자가 수동으로 적지 않아도 컴파일러가 자동으로 수명을 유추해 주는 규칙과 그 한계를 구분한다.
- 구조체와 참조의 관계: 구조체가 다른 데이터의 참조를 필드로 가질 때 수명 주석을 설계하는 방법

## 상황: 가장 긴 상품 이름을 가진 주문 찾기

입력으로 들어온 두 개의 주문(`&Order`) 중 상품 이름(`item_name`)의 길이가 더 긴 주문의 참조를 반환하는 함수 `get_longer_order(order1: &Order, order2: &Order) -> &Order`를 작성

- 단계 1: 컴파일 에러 직접 마주하기
    
    아래 코드를 그대로 작성하고 `cargo check`를 실행해 컴파일 에러 메시지를 확인
    
    ```rust
    fn get_longer_order(order1: &Order, order2: &Order) -> &Order {
        if order1.item_name.len() > order2.item_name.len() {
            order1
        } else {
            order2
        }
    }
    ```
    
- 단계 2: 수명 주석(`'a`) 추가하여 컴파일 통과시키기
    
    ```rust
    // 힌트 구조: fn get_longer_order<'a>(...) -> &'a Order
    ```
    
- 단계 3: 구조체와 수명 주석 결합하기 (심화)
    
    ```rust
    struct FeaturedOrder<'a> {
        // 여기에 Order의 참조를 담는 필드를 정의
    }
    ```
    

## 힌트

- Order 구조체
    
    ```rust
    struct Order {
        id: u32,
        item_name: String,
        price: u32,
        quantity: u32,
        is_delivered: bool,
    }
    ```
    
- 댕글링 참조 방지 원리
    
    댕글링 참조(Dangling Reference)란 이미 메모리에서 해제되었거나 소멸한 데이터의 주소를 가리키고 있는 포인터나 참조. 비정상 종료나 심각한 보안 취약점을 유발한다.
    
    러스트는 Ownership, Scope, Borrow chcker, Lifetimes 등으로 컴파일 단계에서 댕글링 참조를 방지한다. 
    
- 수명 생략 규칙
    
    입력이 하나뿐이거나 메서드일 때 알아서 수명을 채워주며, 입력이 여러개인 경우 등 관계가 모호 할 때는 명식적으로 수명 주석을 사용하면 된다. 코드에서 누가 오래 살아 남아야 하는가의 결정은 개발자의 몫이다. 
    
    ‘`a’ 는 그냥 ‘`a’ 일 뿐이다. ‘`b’ 라고 적어도 된다.
    
    - 1. 입력 수명 규칙 (Input Lifetime Rule)
        
        함수의 각 매개변수 중 참조자인 것들은 각각 고유한 수명 주석을 하나씩 부여받습니다.
        
        - 예: `fn foo(x: &i32, y: &i32)`는 컴파일러가 내부적으로 `fn foo<'a, 'b>(x: &'a i32, y: &'b i32)`로 인식합니다.
    - 2. 단일 입력 수명 규칙 (Single Input Lifetime Rule)
        
        만약 입력 매개변수가 정확히 단 하나뿐이라면, 그 하나의 수명이 모든 반환값의 수명으로 그대로 할당됩니다.
        
        - 예: `fn foo(s: &str) -> &str`는 컴파일러가 자동으로 `fn foo<'a>(s: &'a str) -> &'a str`로 해석합니다. 입력된 참조의 수명이 그대로 반환값까지 이어짐을 보장합니다.
    - 3. 메서드 수명 규칙 (Method Self Lifetime Rule)
        
        메서드의 경우, 매개변수에 `&self`나 `&mut self`가 포함되어 있다면 `self`의 수명이 모든 반환값의 수명으로 할당됩니다.
        

## 결과

- 단계1: 컴파일 에러분석
    - `missing lifetime specifier`
    - 반환하는 참조가 `order1`과 `order2` 중 어느 것과 수명이 일치하는지 컴파일러가 알 수 없기 때문입니다. 함수가 끝난 뒤에도 반환된 참조가 안전하게 유효한지 보장하기 위해 수명 관계를 명시해야 한다.
    
    ```rust
    // 에러가 발생하는 코드
    fn get_longer_order(order1: &Order, order2: &Order) -> &Order {
        if order1.item_name.len() > order2.item_name.len() {
            order1
        } else {
            order2
        }
    }
    ```
    
- 단계2: 명시적 수명 주석(`'a`) 추가
    
    입력 인자와 반환값의 수명이 같거나 최소한 `'a` 기간 동안은 유효함을 컴파일러에게 알려준다.
    
    ```rust
    fn get_longer_order<'a>(order1: &'a Order, order2: &'a Order) -> &'a Order {
        if order1.item_name.len() > order2.item_name.len() {
            order1
        } else {
            order2
        }
    }
    ```
    
- 단계3: 구조체와 수명 주석 결합
    
    구조체가 참조자를 필드로 가질 때는, 그 참조자가 구조체보다 먼저 소멸되지 않도록 구조체 정의 자체에 수명 주석을 명시해 준다.
    
    ```rust
    struct FeaturedOrder<'a> {
        order: &'a Order,
        highlight_reason: String,
    }
    ```
    
- 전체 코드
    
    ```rust
    struct Order {
        id: u32,
        item_name: String,
        price: u32,
        quantity: u32,
        is_delivered: bool,
    }
    
    // 수명 주석을 포함한 비교 함수
    fn get_longer_order<'a>(order1: &'a Order, order2: &'a Order) -> &'a Order {
        if order1.item_name.len() > order2.item_name.len() {
            order1
        } else {
            order2
        }
    }
    
    // 참조자를 필드로 가지는 구조체 (수명 주석 필수)
    struct FeaturedOrder<'a> {
        order: &'a Order,
        highlight_reason: String,
    }
    
    fn main() {
        let order_a = Order {
            id: 1,
            item_name: String::from("Wireless Mouse"),
            price: 30_000,
            quantity: 1,
            is_delivered: true,
        };
    
        let order_b = Order {
            id: 2,
            item_name: String::from("Mechanical Keyboard"),
            price: 120_000,
            quantity: 1,
            is_delivered: false,
        };
    
        // 더 긴 이름을 가진 주문의 참조를 가져옵니다.
        let longer = get_longer_order(&order_a, &order_b);
        println!("더 긴 상품 이름: {}", longer.item_name);
    
        // 구조체에 참조를 안전하게 담습니다.
        let featured = FeaturedOrder {
            order: longer,
            highlight_reason: String::from("가장 긴 상품명 보유"),
        };
    
        println!("특집 주문 선정 이유: {}", featured.highlight_reason);
    }
    ```
    

## 과제

- 상황 설정
    
    기존의 `Order` 구조체 데이터가 있을 때, 원본 데이터를 직접 복사하거나 소유권을 빼앗아 오지 않고 참조(`&Order`)를 안전하게 담아두는 전용 뷰어 구조체를 만들려고 합니다.
    
- 요구사항
    1. 함수 작성 (`find_first_express_order`)
        - 주문 슬라이스(`&[Order]`)를 인자로 받아, 가격이 50,000원을 넘는 첫 번째 주문의 참조를 반환하는 함수를 작성하세요.
        - 반환 타입은 `Option<&Order>` 형태여야 합니다. (조건에 맞는 주문이 없을 수도 있으므로)
        - *힌트*: 입력이 하나이므로 수명 생략 규칙이 적용되지만, 명시적으로 `'a`를 붙여 시그니처를 직접 완성해 보세요.
    2. 구조체 작성 (`OrderViewer`)
        - 원본 주문의 참조를 필드로 품고 있는 구조체 `OrderViewer<'a>`를 정의하세요.
        - 필드 구성:
            - `target_order`: `&'a Order` (찾아낸 주문의 참조)
            - `viewer_note`: `String` (검토 메모)
        - 구조체 정의 시 수명 주석(`'a`)을 올바르게 배치해야 합니다.
    3. `main` 함수 완성
        - `Order` 인스턴스들이 담긴 벡터를 만들고, `find_first_express_order`를 호출하여 얻은 참조를 `OrderViewer`에 안전하게 담아 출력해 보세요.
- 템플릿 코드
    
    ```rust
    struct Order {
        id: u32,
        item_name: String,
        price: u32,
        quantity: u32,
        is_delivered: bool,
    }
    
    // 1. 수명 주석을 명시한 검색 함수 완성하기
    fn find_first_express_order(orders: &[Order]) -> Option<&Order> {
        // 여기에 코드를 작성하세요 (가격이 50000 초과인 첫 번째 주문 찾기)
        todo!()
    }
    
    // 2. 참조를 필드로 가지는 구조체 정의하기
    struct OrderViewer {
        // 여기에 필드를 채워보세요
    }
    
    fn main() {
        let orders = vec![
            Order { id: 1, item_name: String::from("Pen"), price: 2_000, quantity: 5, is_delivered: true },
            Order { id: 2, item_name: String::from("Keyboard"), price: 75_000, quantity: 1, is_delivered: false },
        ];
    
        // 3. 함수를 호출하고 OrderViewer 생성하기
        // let target = ...;
        // let viewer = ...;
        
        // println!("뷰어 대상 상품: {}", viewer.target_order.item_name);
    }
    ```
    

## 과제 풀이

```rust
struct Order {
    id: u32,
    item_name: String,
    price: u32,
    quantity: u32,
    is_delivered: bool,
}

fn find_first_express_order<'a>(orders: &'a [Order]) -> Option<&'a Order> {
    orders.iter().find(|order| order.price > 50_000)
}

struct OrderViewer<'a> {
    target_order: &'a Order,
    viewer_note: String,
}

fn main() {
    let orders = vec![
        Order { id: 1, item_name: String::from("Pen"), price: 2_000, quantity: 5, is_delivered: true },
        Order { id: 2, item_name: String::from("Keyboard"), price: 75_000, quantity: 1, is_delivered: false },
    ];

    if let Some(target) = find_first_express_order(&orders) {
        let viewer = OrderViewer {
            target_order: target,
            viewer_note: String::from("고가 상품 검토 대상"),
        };
        
        println!("뷰어 대상 상품: {}", viewer.target_order.item_name);
        println!("검토 메모: {}", viewer.viewer_note);
        println!("상품 가격: {}원", viewer.target_order.price);
    } else {
        println!("조건에 맞는 주문이 없습니다.");
    }
}
```
