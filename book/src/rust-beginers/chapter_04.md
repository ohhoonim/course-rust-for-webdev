# 데이터 타입(data type)

변수를 선언할 때 타입을 적어주지 않더라도 rust 가 알아서 추론해주지만 데이터타입은 명시적으로 적어주자. 변수 타입은 ‘변수명:타입’ 형태로 적어준다.

## Scalar Types

단 하나의 값을 나타내는 타입이며, 크게 4가지 기본 타입이 있다.

```rust
fn main() {
	let age: u32 = 10;
	let num: i32 = -100;
	let temperature: f64 = 36.6;
	let is_active: bool = true;
	let grade: char = 'A';
	
	println!("age: {}, temp: {}, active: {}, grade: {}, num: {}",
		age, temperature, is_active, grade, num
	);
}
```

- 정수형(Integers)
    - 소수점이 없는 숫자. 부호 유무에 따라 u(unsigned), i(signed)로 구분한다.
    - default: i32
- 부동소수점(Floating-Point Types)
    - 소수점이 있는 숫자
    - default: f32, f64
- 불리언(Boolean)
    - 참/거짓, `bool` 로 선언
    - 값: true / false (1바이트)
- 문자 타입(Character)
    - 단일 문자, `char` 로 선언
    - 4바이트(유니코드 스칼라 값), 이모지도 사용 가능


rust에서는 ‘is_active’와 같이 스네이크 스타일을 사용하는 것이 네이밍 컨벤션(명명 규칙)이다.

## Compound Types

- 튜플(Tuple)
    - 다양한 타입을 소괄호(`()`) 로 묶어준다. (고정길이)
    - 인덱스를 사용하거나 구조 분해(Destructuring)를 이용하여 값을 꺼내온다.
    
    ```rust
    let person: (&str, u32, u32) = ("Maya", 29, 50000);
    println!("{} is {} years old, has the salary {}",
    		person.0, person.1, person.2);
    ```
    
    - 구조분해
    
    ```rust
    
    let person: (&str, u32, u32) = ("Maya", 29, 50000);
    
    let (name, age, salary) = person;
    
    println!("{} is {} years old, has the salary {}", name, age, salary);
    
    ```
    
- 배열(Array)
    - 동일한 타입을 대괄호(`[]`)로 묶어준다. (고정길이)
    - ‘[타입; 길이]’ 형식을 사용한다.
    - 가변 길이의 배열은 `Vec` 를 사용한다.
    
    ```rust
    let scores: [i32; 3] = [90, 85, 100];
    println!("first score: {}", scores[0]);
    ```
    
    - `vec!` 매크로: 동적 배열인 벡터(`Vec<T>`)를 편리하게 사용하게 해준다.
    
    ```rust
    let mut numbers = vec![1, 2, 3, 4, 5]; // 타입추론
    println!("초기 벡터: {:?}", numbers); // 디버그 출력을 위해 {:?} 사용
    
    let zeros = vec![0; 3]; // 특정 값을 특정 길이만큼 반복해서 초기화
    println!("0으로 채워진 벡터: {:?}", zeros);
    ```
    
    
## `&str` (string slice) 

“Hello”처럼 문자열 리터럴을 사용할 때 만들어지는 타입이다.

읽기 전용이며 ``static` 라이프사이클을 가진다. 우선은 이 정도만 알고 있자.
    
    
## `{}` vs `{:?}`의 차이

현시점에서는 이런것도 있다 정도로만 기억하자

- `{}` (`Display` 트레이트)
    - 용도: 일반 사용자에게 보여주기 위한  출력
    - 대상: 문자열, 숫자 등 기본 타입처럼 "어떻게 화면에 보여야 할지"가 명확한 타입들에만 기본 구현되어 있다.(scalar 타입들)
    - 특징: 직접 만든 구조체(Struct)나 벡터(Vec) 등에 그냥 `{}`를 쓰면 컴파일 에러가 발생
- `{:?}` (`Debug` 트레이트)
    - 용도: 개발자가 디버깅을 목적으로 데이터의 내부 구조나 상태를 확인하기 위한 출력
    - 대상: `Vec`, `HashMap`, 직접 만든 `Struct`나 `Enum` 등 복잡한 데이터 구조를 화면에 출력해보고 싶을때 사용
    - 특징: `#[derive(Debug)]`라는 매크로만 붙여주면, 컴파일러가 자동으로 구조체의 내용을 출력할 수 있는 코드를 만들어 준다.
