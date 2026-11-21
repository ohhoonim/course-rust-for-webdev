
# Trait & Generic

## Generic

데이터 타입을 특정하지 않고 매개변수(Placeholder)처럼 다룰수 있게 해주는 기능이다. 관례적으로 <T> 로 명명한다. 러스트는 컴파일시 타입을 전부 풀어주므로 프로그램 실행시오버헤드가 전혀 없다.

```rust
struct Point<T> {
    x: T,
    y: T,
}

fn main() {
    let integer_point = Point { x: 5, y: 10 };       // T가 i32로 결정됨
    let float_point = Point { x: 1.0, y: 4.0 };     // T가 f64로 결정됨
}
```

- 예시: Generic을 사용하지 않았을 때

```rust
fn main() {
    let numbers = vec![10, 20, 54, 12, 4];
    println!("{}", max_value_no_use_generic(&numbers));
}

fn max_value_no_use_generic (list: &[i32]) -> i32 {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    } 
    largest
}
```

- 예시: Generic을 사용 했을때

```rust
fn main() {
    let chars = vec!['a', 'c', 'b', 'a'];
    println!("{}", max_value_use_generic(&chars));
}

fn max_value_use_generic<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    } 
    largest
}
```

- (참고) Trait Bound
    - `T: PartialOrd + Copy` 부분에 쓰인 `PartialOrd`와 `Copy`를 트레이트 바운드(Trait Bounds)라고 한다.
    - `PartialOrd`: 값이 서로 크다 작다를 비교(`>`)할 수 있는 기능(트레이트)이 반드시 구현되어 있어야 함을 의미.
    - `Copy`: 대입 연산 시 값이 복사되도록 하여, `largest = item;`처럼 변수를 할당할 때 소유권이 이동(Move)하지 않고 복사(Copy)될 수 있는 타입이어야 한다는 뜻
    - `+` (플러스 기호): 여러 트레이트 제약을 동시에 걸어줄 때 사용하는 And(그리고) 연산자 (즉, `PartialOrd`와 `Copy`를 둘 다 구현한 타입만 허용).

## Trait

특정 타입이 어떤 메서드를 반드시 구현해야 하는지 규약을 정의한다.  Trait도 타입이므로 제네릭으로 사용할 수 있다.

- 기본사용법
    
    ```rust
    // 요약 기능을 정의하는 트레이트
    trait Summary {
        fn summarize(&self) -> String; // 구현해야 할 메서드 시그니처
    }
    
    struct Article {
        headline: String,
        author: String,
    }
    
    struct Tweet {
    	text: String,
    }
    
    // Article에 Summary 트레이트 구현
    impl Summary for Article {
        fn summarize(&self) -> String {
            format!("{} by {}", self.headline, self.author)
        }
    }
    
    impl Summary for Tweet {
        fn summarize(&self) -> String {
            format!("{}", self.text)
        }
    }
    ```
    

## STD 주요 트레이트

- 마커 및 복제 트레이트 (`std::marker` / `std::clone`)
    - `Copy`: 비트 단위 복사가 가능하여, 대입 시 소유권 이동 대신 복사가 일어나는 타입
    - `Clone`: 명시적인 복제(`clone()`) 메서드를 제공하여 깊은 복사가 가능한 타입
    - `Sized`: 컴파일 타임에 크기를 명확히 알 수 있는 타입 (대부분의 타입이 기본 적용)
    - `Send` / `Sync`: 스레드 간 안전한 소유권 이동(`Send`) 및 참조 공유(`Sync`)가 가능한 타입
- 비교 트레이트 (`std::cmp`)
    - `PartialEq`: 동등성 비교(`==`, `!=`)가 가능한 타입
    - `Eq`: 완전한 동등성 비교가 가능한 타입 (자기 자신과 항상 같음 보장)
    - `PartialOrd`: 크기 비교(`<`, `>`, `<=`, `>=`)가 가능한 타입
    - `Ord`: 완전한 대소 비교가 가능하여 정렬(Sorting)이 가능한 타입
- 변환 및 포맷 트레이트 (`std::convert` / `std::fmt`)
    - `From` / `Into`: 다른 타입으로부터 자기 자신을 생성하거나 변환하는 트레이트
    - `Display`: 사용자 친화적인 문자열 표현을 제공 (`{}` 포맷터 사용)
    - `Debug`: 개발자용 디버깅 문자열 표현을 제공 (`{:?}` 포맷터 사용)
- 연산자 및 메모리 관리 트레이트 (`std::ops`)
    - `Add`, `Sub`, `Mul`, `Div`: 산술 연산자(`+`, , , `/`)를 오버로딩할 수 있는 트레이트
    - `Deref`: 참조자 역참조( 및 자동 역참조) 동작을 커스텀하는 트레이트
    - `Drop`: 값이 스코프를 벗어날 때 실행할 정리 작업(소멸자)을 정의하는 트레이트
    - `Default`: 타입의 기본값(`Default::default()`)을 제공하는 트레이트
- 이터레이터 트레이트 (`std::iter`)
    - `Iterator`: 요소를 순차적으로 순회하며 처리할 수 있는 반복자 동작을 정의하는 트레이트
