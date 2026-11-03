
# Iterators & Closures

## 목표

- 클로저(Closure)
    - 클로저의 구문(Syntax)과 타입 추론 방식을 이해합니다.
    - 환경 캡처 방식에 따른 세 가지 트레이트(`Fn`, `FnMut`, `FnOnce`)의 차이와 소유권 이동 규칙을 파악합니다.
- 이터레이터(Iterator)
    - 이터레이터의 지연 평가(Lazy evaluation) 특성을 이해합니다.
    - `iter()`, `iter_mut()`, `into_iter()`의 차이를 구분하고 상황에 맞게 적용합니다.
    - `map`, `filter`, `fold`, `collect` 등 다양한 어댑터와 소비자 메서드를 조합하여 복잡한 데이터를 선언적으로 처리합니다.

## 상황: 쇼핑몰 주문 로그 분석

일련의 주문 데이터가 들어 있는 벡터를 처리하는 프로그램을 작성합니다. 클로저와 이터레이터를 조합하여 데이터를 필터링하고 통계를 내는 작업을 수행

- 배송 완료된 주문만 필터링 (`filter`): `is_delivered`가 `true`인 주문들만 골라냅니다.
- 총 주문 금액 계산 (`map`과 `fold` 또는 `sum`): 필터링된 각 주문의 가격 곱하기 수량(`price * quantity`)을 계산한 뒤, 총합을 구합니다.
- 고가 상품 리스트 추출 (`filter`와 `map` 및 `collect`): 배송 여부와 상관없이 총 가격(`price * quantity`)이 100,000원을 초과하는 상품들의 이름(`item_name`)만 모아서 새로운 `Vec<String>`으로 수집합니다. 이때 외부 변수(기준 금액 100,000원)를 캡처하는 클로저를 사용해 보세요.

## 힌트

- 기본 데이터 구조
    
    ```rust
    #[derive(Debug)]
    struct Order {
        id: u32,
        item_name: String,
        price: u32,
        quantity: u32,
        is_delivered: bool,
    }
    
    fn main() {
        let orders = vec![
            Order { id: 1, item_name: String::from("Keyboard"), price: 50000, quantity: 2, is_delivered: true },
            Order { id: 2, item_name: String::from("Mouse"), price: 30000, quantity: 1, is_delivered: false },
            Order { id: 3, item_name: String::from("Monitor"), price: 300000, quantity: 1, is_delivered: true },
            Order { id: 4, item_name: String::from("USB Cable"), price: 5000, quantity: 5, is_delivered: true },
            Order { id: 5, item_name: String::from("Headset"), price: 80000, quantity: 2, is_delivered: false },
        ];
    
        // TODO: 미션 수행
    ```
    
- 지연평가
    
    러스트의 이터레이터 어댑터(`map`, `filter` 등)들은 실제로 데이터를 처리하지 않는다. 대신 처리할 파이프라인(작업목록)만 쌓아둔다. 실제로 실행되는 시점은 컨슈머(Consumer)(`.collect()`, `.sum()`, `.for_each()`, `.count()` 등)가 호출되었을 때이다.
    
    참고로, 자바 스트림에서는 어댑터를 중간연산(Intermediate Operation), 컨슈머를 종단 연산(Terminal Operation)이라고 한다. 
    
- 클로저(Closure)
    - Syntax
        - 기본 형태: `|x, y| x + y`
        - 타입 명시: 기본적으로 타입은 자동으로 추론되며, 필요하다면  타입을 적어도 된다.(예: `|x: i32, y: i32| -> i32 { x + y }`)
        - 블록 본문: 코드가 한 줄이면 중괄호 `{}`를 생략할 있다.
    - 클로저의 구분 (Trait): 컴파일러의 자동 유추 규칙에 의해 결정됨.
        - Fn: 불변 참조로 캡쳐. &T
        - FnMut: 가변 참조로 캡처. &mut T
        - FnOnce: 소유권 이동. T
    - `move` 키워드
        
        컴파일러의 자동 유추 규칙과 상관없이 명시적으로 소유권 이전을 강제
        
        ```rust
        let message = String::from("안녕하세요, 멀티스레드!");
        
        // move 키워드를 사용하여 message의 소유권을 스레드 안으로 이동
        let handle = thread::spawn(move || {
            println!("스레드 내부에서 메시지 출력: {}", message);
            // message의 소유권이 이 클로저(스레드)로 넘어왔으므로 여기서 자유롭게 사용 가능
        });
        ```
        
- 역참조
    - `*` 연산자: 참조가 가리키는 실제 메모리 공간의 값에 접근합니다.
    - 패턴 매칭을 통한 역참조: 클로저 매개변수에서 `|&n|` 형태로 패턴 매칭을 사용하면, 자동으로 참조를 풀어서 내부 값을 가져올 수도 있다. (예: `|n| *n`과 `|&n|`은 동일하게 동작)
    - 점 연산자(`.`)의 자동 역참조: 점(`.`) 연산자는 구조체의 필드에 접근하거나 메서드를 호출할 때, 컴파일러가 알아서 필요한 만큼 역참조(`*`)를 자동으로 반복해 주는 편의 기능을 제공
    
    ```rust
    fn main() {
        let x = 5;
        let y = &x; // y는 i32의 불변 참조자(&i32)이다.
    
        // x는 값 5이고, y는 주소를 담고 있으므로 직접 비교하면 에러남.
        // * 연산자를 사용해 y가 가리키는 실제 값(역참조)을 꺼내어 비교
        assert_eq!(5, x);
        assert_eq!(5, *y); 
    
        println!("y가 가리키는 실제 값: {}", *y);
    
        // 이터레이터와 결합된 역참조 예시
        let numbers = vec![1, 2, 3];
    
        // .iter()는 &i32 형태의 참조 이터레이터를 생성
        let doubled: Vec<i32> = numbers
            .iter()
            .map(|n| *n * 2) // n은 &i32 타입이므로 *n을 통해 실제 값을 꺼내야 함
            .collect();
    
        println!("결과: {:?}", doubled); // 출력: [2, 4, 6]
    }
    ```
    

## 결과

- 배송 완료된 주문만 필터링
    
    ```rust
    let delivered_orders: Vec<&Order> = orders
            .iter()
            .filter(|order| order.is_delivered)
            .collect();
    ```
    
- 총 주문 금액 계산
    
    ```rust
    let total_revenue: u32 = orders
            .iter()
            .filter(|order| order.is_delivered)
            .map(|order| order.price * order.quantity)
            .sum();
    ```
    
- 고가 상품 리스트 추출
    
    ```rust
    let expensive_items: Vec<String> = orders
            .iter()
            .filter(|order| order.price * order.quantity > threshold)
            .map(|order| order.item_name.clone())
            .collect();
    ```
    

## 과제

- 과제 1: `iter_mut()`를 이용한 일괄 할인 적용
    - 목표: 가변 참조를 다루는 이터레이터와 클로저를 사용하여 데이터 직접 수정하기
    - 내용: `.iter_mut()`를 사용해 아직 배송되지 않은(`is_delivered == false`) 모든 주문의 가격(`price`)을 10% 할인된 값(`price * 90 / 100`)으로 수정하고, 전체 주문 목록을 출력하는 코드를 작성하세요.
- 과제 2: `into_iter()`와 소유권을 활용한 데이터 변환
    - 목표: 소유권을 이전하는 이터레이터를 이해하고 새로운 구조체 형태로 가공하기
    - 내용: 기존의 `orders` 벡터를 `.into_iter()`로 소비하면서, 각 주문을 아래의 요약 구조체로 변환(`map` 및 `collect`)하는 코드를 작성하세요.
        
        ```rust
        struct OrderSummary {
            id: u32,
            description: String, // 예: "Keyboard (2개)" 형태로 포맷팅
        }
        ```
        
- 과제 3: `any`와 `max_by_key`를 활용한 조건 검사와 집계
    - 목표: 단일 값이나 조건 만족 여부를 반환하는 소비자(Consumer) 메서드 사용하기
    - 내용: 클로저를 활용해 다음 동작을 수행하는 코드를 각각 작성하세요.
        1. `.any()`를 사용하여 수량이 5개 이상인 주문이 하나라도 존재하는지 확인(`bool` 반환)
        2. `.max_by_key()`를 사용하여 총 주문 금액(`price * quantity`)이 가장 큰 주문 객체를 찾아내기

## 과제 풀이

- 과제 1: `iter_mut()`를 이용한 일괄 할인 적용
    
    ```rust
    // 배송되지 않은 주문의 가격을 10% 할인
      orders
          .iter_mut()
          .filter(|order| !order.is_delivered)
          .for_each(|order| {
              order.price = order.price * 90 / 100;
          });
    ```
    
    `.iter_mut()`를 사용하여 가변 참조(`&mut Order`)를 얻고, `.filter()`로 배송 전인 주문을 골라낸 뒤, 소비 메서드인 `.for_each()` 안에서 클로저를 통해 원본 구조체의 필드 값을 직접 수정(`order.price = ...`)합니다.
    
- 과제 2: `into_iter()`와 소유권을 활용한 데이터 변환
    
    ```rust
    let summaries: Vec<OrderSummary> = orders
            .into_iter()
            .map(|order| OrderSummary {
                id: order.id,
                description: format!("{} ({}개)", order.item_name, order.quantity),
            })
            .collect();
    ```
    
    - `.into_iter()`를 사용하여 `orders` 벡터의 소유권을 통째로 가져옵니다.
    - `.map()` 안에서 각 `Order`의 필드 소유권을 가져와(`order.item_name` 등) 새로운 `OrderSummary` 구조체로 변환하고, `.collect()`로 모아줍니다.
- 과제 3: `any`와 `max_by_key`를 활용한 조건 검사와 집계
    
    ```rust
    // 1. 수량이 5개 이상인 주문이 하나라도 있는지 확인
    let has_bulk_order = orders
        .iter()
        .any(|order| order.quantity >= 5);
    
    // 2. 총 주문 금액이 가장 큰 주문 찾기
    let most_expensive_order = orders
        .iter()
        .max_by_key(|order| order.price * order.quantity);
    ```
    
    - `.any()`는 클로저의 조건이 참이 되는 요소가 하나라도 있으면 즉시 `true`를 반환합니다.
    - `.max_by_key()`는 클로저가 반환하는 비교 기준값(`price * quantity`)을 바탕으로 가장 큰 값을 가진 요소를 찾아 `Option<&Order>` 형태로 반환합니다.
