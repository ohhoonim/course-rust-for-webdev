
# Option과 Result

## (참고) Generic

코드에서 Option&lt;String&gt; 과 같이 꺽쇠로 표현하는 것을 ‘Generic’이라고 한다. Trait & Generic 에서 설명할 에정.


## Option

값이 있을 수도, 없을 수도 있음을 나타내는 표준 enum. (null pointer 방지)

```rust
fn main() {
    let user = find_uesr(12);
    let username = match user {
        Some(u) => u,
        None =>  "찾을 수 없습니다.".to_string(),
    };
    println!("{}", username);

}

fn find_uesr(id: i32) -> Option<String> {
    if id == 11 {
        Some("matthew".to_string())
    } else {
        None
    }
}

```

## Result

작업이 성공과 실패를 반환할 수 있도록하는 표준 enum. 에러 처리를 할 수 있게 도와준다. (실패한 원인을 전달하고 싶을때 사용). 결과에 대한 정상 타입과 에러 타입을 구분하여 지정한다.

```rust
fn main() {
	let result = divide(20.0, 0.0);
    match result {
        Ok(r) => println!("{}", r),
        Err(m) => println!("{}", m),
    }
}

fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        return Err("0으로 나눌 수 없다".to_string())
    }
    Ok(a / b)
}

```
