
# Smart Pointers: Box, Rc, RefCell️

## 목표

- 힙 메모리 할당과 크기 제어 (`Box<T>`): 컴파일 타임에 크기를 알 수 없는 재귀적 타입이나 큰 데이터를 힙에 안전하게 할당하고 관리하는 법
- 단일 스레드 공유 소유권 (`Rc<T>`): 하나의 데이터를 여러 곳에서 동시에 소유해야 할 때 참조 카운팅(Reference Counting)을 통해 안전하게 공유하는 법을 파악
- 내부 가변성(`RefCell<T>`): 대여 규칙(Borrowing Rules)을 컴파일 타임이 아닌 런타임에 검사하여, 불변 참조 상태에서도 내부 값을 변경할 수 있는 패턴 이해
- 스마트 포인터 조합 (`Rc<RefCell<T>>`): 실무에서 자주 쓰이는 공유 + 가변 상태 조합의 패턴

## 상황: 할인 정책과 이벤트 로그를 공유하는 주문 시스템

요구사항

- 1. 재귀적 번들 상품 구조 구현 (`Box<T>`)
    - 요구사항: `Order` 구조체는 선택적으로 또 다른 하위 주문(`sub_order`)을 가질 수 있어야 합니다.
    - 제약 조건: 러스트 컴파일러가 구조체의 크기를 컴파일 시점에 확정할 수 있도록, 하위 주문 필드는 반드시 힙에 할당되는 포인터 타입(`Box<Order>`)으로 감싸야 합니다.
- 2. 쿠폰 공유 소유권 관리 (`Rc<T>`)
    - 요구사항: 여러 개의 독립적인 `Order` 인스턴스가 단 하나의 `DiscountCoupon` 객체를 중복 생성 없이 동시에 참조할 수 있어야 합니다.
    - 제약 조건: 쿠폰 객체를 `Rc<T>`로 감싸고, 새로운 주문이 쿠폰을 참조할 때마다 `Rc::clone`을 호출하여 참조 카운트가 정상적으로 증가하는지 확인할 수 있어야 합니다.
- 3. 런타임 내부 가변성 적용 (`RefCell<T>`)
    - 요구사항: 쿠폰을 공유하고 있는 여러 주문 중 어느 한쪽에서 결제가 완료되면, 공유된 쿠폰의 남은 사용 횟수(`remaining_uses`)를 프로그램 실행 중에 즉시 차감할 수 있어야 합니다.
    - 제약 조건: 쿠폰 변수 자체가 불변(`let`)으로 선언되어 있더라도, `RefCell<T>`의 `.borrow_mut()` 메서드를 통해 런타임에 안전하게 가변 대여를 획득하여 값을 수정할 수 있어야 합니다.
- 4. 상태 동기화 검증
    - 요구사항: 하나의 주문을 통해 쿠폰 사용 횟수가 변경되었을 때, 다른 주문이나 외부에서 동일한 쿠폰을 조회(`borrow()`)했을 때 변경된 값이 정상적으로 반영(공유)되어 있는지 출력(Validation)을 통해 확인할 수 있어야 합니다.

## 힌트

- structs: Order, DiscountCoupon, ‘???’ 이부분을 채우세요
    
    
    ```rust
    #[derive(Debug)]
    struct DiscountCoupon {
        code: String,
        remaining_uses: i32,
    }
    
    // 1. 재귀적 구조체 (Box를 사용하여 힙에 할당)
    #[derive(Debug)]
    struct Order {
        id: u32,
        item_name: String,
        // 1. 번들 상품: 내부에 또 다른 주문을 포함할 수 있음
        sub_order: Option<???>,
        // 2 & 3. 여러 주문이 쿠폰을 공유하고 상태를 변경할 수 있음
        coupon: Option<???>,
    }
    ```
    
- 상황 1: `Box<T>`와 재귀적 구조 (번들 상품)
    
    러스트의 구조체는 컴파일 타임에 정확한 크기(바이트 수)를 알 수 있어야 합니다. 하지만 상품 안에 또 다른 상품 구조체가 무한히 포함될 수 있는 재귀적 구조를 만들면 크기를 정할 수 없어 에러가 발생합니다.
    
    - 힌트: 컴파일 타임에 크기가 불확실한 데이터를 힙 메모리로 밀어내고, 구조체 안에는 고정된 크기의 주소값(포인터)만 남기려면 `Box<T>`로 감싸야 합니다. 포인터의 크기는 고정되어 있으므로 컴파일러가 크기를 계산할 수 있게 됩니다.
- 상황 2: `Rc<T>`와 단일 스레드 공유 소유권 (할인 쿠폰)
    
    러스트의 기본 원칙은 하나의 데이터는 정확히 하나의 소유자만 가질 수 있다는 것입니다. 하지만 여러 주문서가 동일한 쿠폰 데이터를 복사 없이 동시에 참조해야 할 때가 있습니다.
    
    - 힌트: 데이터를 여러 곳에서 안전하게 공유하되, 누군가 참조를 포기할 때마다 카운트를 세어 마지막 소유자가 사라질 때 메모리를 해제해 주는 `Rc<T>`(Reference Counted)를 사용해 보세요. 변수를 복사할 때 데이터 자체가 아니라 `Rc` 자체를 `.clone()`하면 참조 카운트만 올라갑니다.
- 상황 3: `RefCell<T>`와 내부 가변성 (쿠폰 수량 변경)
    
    `Rc<T>`를 사용하면 여러 곳에서 데이터를 공유할 수 있지만, 기본적으로 불변(Immutable) 상태로만 공유됩니다. 만약 공유받은 곳 중 한 곳에서 쿠폰의 남은 수량을 깎는 등의 변경을 가하려고 하면 컴파일 에러가 발생합니다.
    
    - 힌트: 대여 규칙 검사를 컴파일 타임이 아니라 프로그램 실행 중(런타임)으로 미루어주는 `RefCell<T>`를 `Rc`내부에 중첩(`Rc<RefCell<T>>`)해서 사용해 보세요. `.borrow_mut()`를 통해 불변 참조 상태에서도 값을 안전하게 변경할 수 있습니다.
- ref 키워드
    
    러스트에서 `ref` 키워드는 패턴 매칭 시 소유권을 이동(Move)시키는 대신, 참조자(`&`) 형태로 값을 빌려오고 싶을 때 사용하는 키워드입니다.
    
    러스트의 기본 규칙에 따라 `match`나 `if let` 등으로 구조체나 열거형의 내부 값을 패턴 매칭하면, 기본적으로 값의 소유권이 변수로 이동합니다. 이 경우 원본 객체의 소유권이 깨지거나 복사할 수 없는 타입일 경우 컴파일 에러가 발생합니다.
    
    이때 `ref`를 사용하면 원본의 소유권을 그대로 둔 채, 안전하게 참조(`&`) 형태의 바인딩을 만들어 낼 수 있습니다.
    
    - 종류
        - `ref`: 불변 참조(`&T`)를 가져옵니다.
        - `ref mut`: 가변 참조(`&mut T`)를 가져옵니다. 내부 값을 수정해야 할 때 주로 사용합니다.
    
    최신 러스트에서는 굳이 패턴 내부에 `ref`를 적지 않고, 대입하려는 대상 앞에 `&`를 붙이는 방식을 더 권장하기도 합니다.
    
    ```rust
    // ref 키워드를 쓰는 방식
    if let Some(ref coupon_ref) = order1.coupon { ... }
    
    // 오른쪽에 &를 붙이는 현대적인 방식 (결과적으로 동일하게 참조를 얻음)
    if let Some(coupon_ref) = &order1.coupon { ... }
    ```
    

## 결과

```rust
use std::rc::Rc;
use std::cell::RefCell;

// 3. 공유할 할인 쿠폰 구조체 (내부 가변성을 위해 RefCell 사용)
#[derive(Debug)]
struct DiscountCoupon {
    code: String,
    remaining_uses: i32,
}

// 1. 재귀적 구조체 (Box를 사용하여 힙에 할당)
#[derive(Debug)]
struct Order {
    id: u32,
    item_name: String,
    // 번들 상품: 내부에 또 다른 주문을 포함할 수 있음 (크기가 불확실하므로 Box 필수)
    sub_order: Option<Box<Order>>,
    // 2 & 3. 여러 주문이 쿠폰을 공유하고 상태를 변경할 수 있도록 Rc<RefCell<T>> 조합 사용
    coupon: Option<Rc<RefCell<DiscountCoupon>>>,
}

fn main() {
    // 공통으로 사용할 할인 쿠폰 생성 (Rc와 RefCell로 감싸기)
    let shared_coupon = Rc::new(RefCell::new(DiscountCoupon {
        code: String::from("SUMMER_SPECIAL"),
        remaining_uses: 2,
    }));

    // 첫 번째 주문 
    let order1 = Order {
        id: 1,
        item_name: String::from("Laptop"),
        sub_order: Some(Box::new(Order {
            id: 101,
            item_name: String::from("Wireless Mouse"),
            sub_order: None,
            coupon: None,
        })),
        coupon: Some(Rc::clone(&shared_coupon)),
    };

    // 두 번째 주문 생성 (동일한 쿠폰 공유)
    let order2 = Order {
        id: 2,
        item_name: String::from("Mechanical Keyboard"),
        sub_order: None,
        coupon: Some(Rc::clone(&shared_coupon)), 
    };

    println!("--- 초기 주문 정보 ---");
    println!("주문 1: {:#?}", order1);
    println!("주문 2: {:#?}", order2);
    println!("현재 쿠폰 참조 카운트: {}", Rc::strong_count(&shared_coupon));

    // 런타임에 order1을 통해 공유된 쿠폰의 남은 사용 횟수 변경 (내부 가변성 활용)
    if let Some(ref coupon_ref) = order1.coupon {
        // borrow_mut()를 사용해 런타임에 가변 대여를 얻어 값을 수정합니다.
        coupon_ref.borrow_mut().remaining_uses -= 1;
        println!("\n[주문 1에서 쿠폰 사용 완료]");
    }

    // order2에서 바라본 쿠폰의 남은 횟수도 함께 변경되었는지 확인
    if let Some(ref coupon_ref) = order2.coupon {
        println!("주문 2에서 확인한 쿠폰 남은 횟수: {}", coupon_ref.borrow().remaining_uses);
    }
}
```

## 과제

- 상황 설정
    
    온라인 쇼핑몰 시스템을 고도화하고 있습니다. 상품의 계층형 카테고리(상위/하위 카테고리)를 표현하기 위해 재귀적 구조가 필요하며, 여러 주문서가 동일한 실시간 재고 객체를 공유하면서 결제 시 수량이 동기화되어야 합니다.
    
- 요구사항
    1. 재귀적 카테고리 구조체 (`Box<T>`)
        - 이름(`name: String`)과 선택적 하위 카테고리(`sub_category: Option<Box<Category>>`)를 필드로 가지는 `Category` 구조체를 정의하세요.
        - 상위 카테고리 안에 하위 카테고리를 중첩하여 생성할 수 있어야 합니다.
    2. 공유 재고 구조체 정의 (`Rc<RefCell<T>>`)
        - 상품명(`item_name: String`)과 남은 수량(`stock_count: i32`)을 필드로 가지는 `Inventory` 구조체를 정의하세요.
    3. 주문 및 재고 차감 로직 구현
        - `Order` 구조체는 주문 ID(`id: u32`)와 공유 재고 참조(`inventory: Rc<RefCell<Inventory>>`)를 필드로 가집니다.
        - 서로 다른 두 주문(`order1`, `order2`)이 하나의 `Inventory` 객체를 공유하도록 생성하세요.
        - `order1`을 통해 재고의 수량을 1개 차감하고, `order2`에서 조회를 통해 수량이 정상적으로 반영되었는지 확인하세요.
- 템플릿 코드 (이 코드를 완성해 보세요)
    
    ```rust
    use std::rc::Rc;
    use std::cell::RefCell;
    
    // 1. 재귀적 카테고리 구조체 구현하기
    #[derive(Debug)]
    struct Category {
        name: String,
        // 여기에 Box를 사용한 하위 카테고리 필드를 정의하세요
    }
    
    // 2. 공유 재고 구조체 구현하기
    #[derive(Debug)]
    struct Inventory {
        item_name: String,
        stock_count: i32,
    }
    
    // 3. 주문 구조체 구현하기
    #[derive(Debug)]
    struct Order {
        id: u32,
        // 여기에 Rc<RefCell<Inventory>>를 사용한 재고 공유 필드를 정의하세요
    }
    
    fn main() {
        // 1. 카테고리 트리 구축 (Box 활용)
        let root_category = Category {
            name: String::from("Electronics"),
            // sub_category 설정하기 (Laptop -> Accessories)
        };
    
        // 2. 공유 재고 생성 (Rc와 RefCell 조합 활용)
        let shared_inventory = Rc::new(RefCell::new(Inventory {
            item_name: String::from("Mechanical Keyboard"),
            stock_count: 5,
        }));
    
        // 3. 두 개의 주문이 동일한 재고를 공유하도록 생성하기
        let order1 = Order {
            id: 1,
            inventory: Rc::clone(&shared_inventory),
        };
    
        let order2 = Order {
            id: 2,
            inventory: Rc::clone(&shared_inventory),
        };
    
        // 4. order1을 통해 재고 수량 1 감소시키기 (borrow_mut 활용)
        // 여기에 재고 차감 코드 작성
    
        // 5. order2에서 남은 재고 확인하기 (borrow 활용)
        // 여기에 남은 재고 출력 코드 작성
    }
    ```
    

## 과제 풀이

```rust
use std::rc::Rc;
use std::cell::RefCell;

// 1. 재귀적 카테고리 구조체 구현하기 (Box 사용)
#[derive(Debug)]
struct Category {
    name: String,
    sub_category: Option<Box<Category>>,
}

// 2. 공유 재고 구조체 구현하기
#[derive(Debug)]
struct Inventory {
    item_name: String,
    stock_count: i32,
}

// 3. 주문 구조체 구현하기 (Rc<RefCell<Inventory>> 사용)
#[derive(Debug)]
struct Order {
    id: u32,
    inventory: Rc<RefCell<Inventory>>,
}

fn main() {
    // 1. 카테고리 트리 구축 (Box를 사용해 하위 카테고리를 힙에 할당)
    let root_category = Category {
        name: String::from("Electronics"),
        sub_category: Some(Box::new(Category {
            name: String::from("Laptop & PC"),
            sub_category: Some(Box::new(Category {
                name: String::from("Accessories"),
                sub_category: None,
            })),
        })),
    };

    println!("--- 카테고리 구조 ---");
    println!("{:#?}", root_category);

    // 2. 공유 재고 생성 (Rc와 RefCell 조합 활용)
    let shared_inventory = Rc::new(RefCell::new(Inventory {
        item_name: String::from("Mechanical Keyboard"),
        stock_count: 5,
    }));

    // 3. 두 개의 주문이 동일한 재고를 공유하도록 생성하기 (Rc::clone으로 참조 카운트 증가)
    let order1 = Order {
        id: 1,
        inventory: Rc::clone(&shared_inventory),
    };

    let order2 = Order {
        id: 2,
        inventory: Rc::clone(&shared_inventory),
    };

    // 4. order1을 통해 재고 수량 1 감소시키기 (borrow_mut 활용)
    order1.inventory.borrow_mut().stock_count -= 1;
    println!("\n[주문 1 결제 완료: 재고 1개 차감]");

    // 5. order2에서 남은 재고 확인하기 (borrow 활용)
    println!("주문 2에서 확인한 남은 재고 수량: {}개", order2.inventory.borrow().stock_count);
    println!("현재 공유 재고 상태: {:#?}", shared_inventory.borrow());
}
```
