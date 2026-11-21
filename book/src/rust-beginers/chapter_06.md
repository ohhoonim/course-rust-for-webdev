# 제어 흐름(Control Flow)

## 조건 if / else

```rust
fn main() {
    let number = 6;

    // 일반적인 if-else 분기
    if number % 4 == 0 {
        println!("4의 배수입니다.");
    } else if number % 3 == 0 {
        println!("3의 배수입니다.");
    } else {
        println!("그 외의 숫자입니다.");
    }

    // 표현식으로 사용하는 if (let 바인딩)
    let condition = true;
    let x = if condition { 5 } else { 6 }; // x는 5가 됩니다.
    println!("x의 값: {}", x);
}
```

## 반복 loop, while, for

```rust
fn main() {
    // loop
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2; // 루프를 끝내며 20을 반환
        }
    };
    println!("결과: {}", result);

    // while 
    let mut n = 3;
    while n > 0 {
        println!("{}!", n);
        n -= 1;
    }

    // for 
    for i in 0..3 {
        println!("현재 숫자: {}", i);
    }
}
```

## 패턴매칭 match

```rust
fn main() {
    let coin = 25;

    match coin {
        1 => println!("페니!"),
        10 => println!("다임!"),
        25 => println!("쿼터!"),
        other => println!("기타 동전: {}", other), // 나머지 모든 경우
    }
}
```
