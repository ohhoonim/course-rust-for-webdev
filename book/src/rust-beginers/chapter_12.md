
# Collections: Vec, String, HashMap

## Rust 표준라이브러리와 용어 정리

rust에서 제공하는 표준라이브러리를 std라고 표현한다. 그 중 데이터 구조들은 `std::collections` 모듈에 모여있다.  

- 패키지 (Package): 하나 이상의 크레이트를 빌드, 테스트, 공유할 수 있도록 묶어주는 단위입니다. `Cargo.toml` 파일 하나가 하나의 패키지를 관리한다.
- 크레이트 (Crate): 컴파일하는 최소 단위. 라이브러리 크레이트나 실행 가능한 바이너리 크레이트가 될 수 있다. (`std` 는 표준 라이브러리 크레이트이다).
- 모듈 (Module): 크레이트 내부에서 코드를 논리적으로 그룹화하고 공개 범위를 제어하는 단위입니다 (`mod` 키워드로 선언)

## (참고) collection 분류

- 시퀀스(Sequences)
    - 순서가 있는 데이터 모음. 인덱스를 사용
    - Vec, VecDeque, LinkedList 등
- 맵(Maps)
    - 키와 값을 동시에 저장하는 방식
    - HashMap, BTreeMap 등
- 셋(Sets)
    - 순서를 따지지 않는 데이터 모음. 중복을 허용하지 않는다.
    - HashSet, BTreeSet 등

## Vec

동일 타입의 데이터들을 메모리에 연속적으로 저장하고 크기를 변경할 수 있다.

```rust
let mut scores: Vec<i32> = Vec::new();
v.push(10);
v.push(20);
let first = v.get(0); 
```

- 배열과 비교: 배열은 길이를 고정

```rust
let mut scores: [i32; 3] = [10, 20, 30];
```

- 루프 사용시 소유권 이전여부를 충분히 생각하기

```rust
let mut scores: Vec<i32> = Vec::new();
scores.push(10);
scores.push(20);
scores.push(30);
scores.push(40);

for score in &scores {
	println!("{}", score);
}

println!("{}", scores[0]);
```

## String

- `String` 은 Vec<u8>의 래퍼형태이다.
- UTF-8 텍스트 컬렉션이며, `&str` (문자열 리터럴)과는 구분한다.

```rust
let mut s = String::from("Hello");
s.push_str(", Rust!");
```

- `&str` 과 구분하기

```rust
let owned = String::from("hello"); // owned 변수는 "hello"를 소유하고 있다.
let borrowed: &str = "world";

println!("{} {}", owned, borrowed);
```

## HashMap

키와 값을 쌍으로 저장한다. (Dictionary)

```rust
let mut scores: HashMap<String, i32> = HashMap::new();
scores.insert(String::from("math"), 90);
scores.insert(String::from("korean"), 98);

if let Some(score) = scores.get("korean") {
    println!("{}", score);
}

for (subject, score) in &scores {
    println!("subject: {}, score: {}", subject, score);
}
```