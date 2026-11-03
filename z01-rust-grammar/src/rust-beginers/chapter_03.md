# 변수와 가변성

러스트(Rust)에서 변수(Variable)와 가변성(Mutability)은 메모리 안전성을 보장하기 위한 핵심 철학 중 하나이다. Rust에서 변수는 기본적으로 값을 변경할 수 없는 '불변(Immutable)'이다.

## 불변 변수 (Immutable Variables)

러스트에서 `let` 키워드로 변수를 선언하면, 기본적으로 그 값은 변경할 수 없다(Read-only). 컴파일러가 값이 변하지 않는다는 것을 보장하므로, 멀티스레드 환경 등에서 안전하게 코드를 최적화하고 버그를 예방할 수 있다.

```rust
fn main() {
    let x = 5;
    // x = 6; // 에러
}
```

## 가변 변수 (Mutable Variables)

만약 변수의 값을 나중에 바꾸고 싶다면, 변수를 선언할 때 `mut` 키워드를 명시하여 가변 변수로 만들어야 한다.

```rust
fn main() {
    let mut x = 5; 
    println!("초기값: {}", x);

    x = 6; // 정상 작동!
    println!("변경된 값: {}", x);
}
```

## 새도잉(Shadowing)

기존 변수의 이름을 재사용하여 새로운 변수를 선언할 수 있다. 이를 섀도잉(Shadowing) 이라고 한다. 이름만 같을 뿐 새로운 변수를 만드는 것이니 가변 변수와 구분하자.  

```rust
fn main() {
    let x = 5;
    let x = x + 1; // 기존 x를 가리키는 새로운 x 선언 (값은 6이 됨)

}
```

## 상수(const)

프로그램 실행 동안 절대 바뀌지 않는 고정된 값. `const` 키워드를 사용하고, 변수의 타입을 반드시 명시해야 한다. 당연하지만, `mut`를 붙일 수 없으며 shadowing을 할 수 없다.

```rust
// (타입 필수, 대문자 권장)
const MAX_POINTS: u32 = 100_000;
const PI: f64 = 3.14159;

fn main() {
    println!("최대 포인트: {}", MAX_POINTS);
    println!("파이 값: {}", PI);
}
```

