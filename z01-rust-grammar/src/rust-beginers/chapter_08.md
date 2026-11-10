
# Borrow와 References

## move: 소유권 이동

```rust
fn main() {
	let s1 = String::from("hello");
	let len = calculate_length(s1);
	println!("{} {}", s1, len); // <- error 's1'
}

fn calulate_length(s: String) -> usize {
	s.len()
}
```

- s1을 calculate_length 함수의 파라미터로 사용하면 s1의 소유권이 calculate_length  함수로 이전한다.
- 이를 소유권이 이동(move) 되었다고 한다.
- 위 코드에서 s1의 소유가 이동되어 “hello”를 가리키는 참조주소가 없어졌으므로 에러가 난다.

## borrow: 소유권 빌리기(메모리 주소만 잠깐 사용하기)

변수명에 `&` 기호를 붙여준다. 가변 변수는 `&mut` 를 사용한다. (references: 참조  참고)

```rust
fn main() {
	let s1 = String::from("hello");
	let len = calculate_length(&s1); // s1을 빌려온다.
	println!("{} {}", s1, len); // <- ok!!
}

fn calulate_length(s: &String) -> usize {
	s.len()
}
```

- “hello” 값을 가리키는 s1의 참조주소만 calculate_length 함수인자로 전달한다.
- caculate_length 함수가 일을 끝내면 참조주소를 다시 s1에게 돌려준다.


## usize

부호가 없는 정수( 0 이상). 32비트 시스템에서는 32비트(4바이트) 크기를 가지며, 64비트 시스템에서는 64비트(8바이트) 크기이다. 

- 배열이다 벡터의 인덱스는 무조건 usize이다.
- 컬렉션의 길이(len())나 크기(capacity())는 usize이다.
- usize는 메모리에 담을 수 있는 최대 개수이다. (포인터 크기와 일치)

## references: 참조

- `&` (불변 참조, Immutable Reference)
    - 용도: 데이터를 읽기만 할 때 사용(기본값)
    - 특징: 원본 데이터를 수정할 수 없지만, 동시에 여러 개의 불변 참조를 만드는 것 허용
- `&mut` (가변 참조, Mutable Reference)
    - 용도: 빌려온 데이터를 수정(변경)해야 할 때 사용
    - 특징: 데이터를 직접 바꿀 수 있기 때문에, 러스트에서는 데이터 경쟁(Data Race)을 막기 위해 아주 엄격한 규칙을 적용
    
    
    
## 가변참조 사용시 주의 사항

- 이미 만들어진 &mut가 존재하면 추가적인 참조자를 만들 수 없다
- 불변 참조(`&`)들이 이미 여러 개 존재하고 있다면, 그 데이터의 가변 참조(`&mut`)를 만들 수 없다 (읽고 있는데 갑자기 고치면 안 되기 때문)

```rust
fn main() {
    let mut s = String::from("hello");
    add_exclamation(&mut s);
    println!("{}", s);
}

fn add_exclamation(text: &mut String) {
    text.push.str("!");
}
```
   