
# 구조체(Stucts)

러스트에서 관련 속성들을 하나로 묶어 사용자 정의 타입을 만드는 기능. 타 언어의 class와 유사하지만 상속(inheritance)이 없는 것이 특징이다. 

## 필드 구조체

가장 일반적인 형태. 필드에 이름을 부여한다.

```rust
#[derive(Debug)] // 디버그 프린트 기능을 자동으로 만들어준다.
struct User {
	username: String,
	email: String,
	active: bool,
}
```

## 튜플 구조체

필드 명을 부여하지 않고 타입 순서만 지정하는 구조체

```rust
struct Color(i32, i32, i32); // RGB 색상
```

## 유닛 유사 구조체

필드가 없는 구조체, 트레이트를 구현할 때만 사용하려는 경우 주로 사용

```rust
struct Empty;
```

## 구조체 기본 사용법

```rust
#[derive(Debug)]
struct User {
	username: String,
	age: u32,
	active: bool,
}

fn main() {
	let matthew = Uer {
		username: "matthew",
		age: 22,
		active: false
	}
	
	println!("{}", matthew);
}
```

## 구조체에 메서드 추가하기

`impl` 키워드를 사용하고, 첫번째 인자를  `self` 로 받는다.(또는 &self, &mut self ) 

```rust
struct User {
	username: String,
	age: u32,
	active: bool,
}

impl User {
	fn describe(&self) -> String {
		format!("{} (active: {})", self.username, self.active))
	}
}
```

- 사용법

```rust
let matthew = User {
        username : "matthew".to_string(),
        age: 22,
        active: false,
    };

matthew.describe();
```



## self vs Self

self: 인스턴스를 의미한다.

Self: 타입을 의미한다. 주로 리턴 타입이 자기 자신일때 사용한다.


## 구조체에 연관 함수 추가하기

첫 번째 인자로 `self` 를 사용하지 않는다. 주로 생성자 용도로 많이 사용된다.( 타 언어의 static 메서드와 유사)

```rust
struct User {
	username: String,
	age: u32,
	active: bool,
}

impl User {
	fn new() -> Self {
		User {
			username: "",
			age: 0,
			active: false,
		}
	}
	fn describe(&self) -> String {
		format!("{} (active: {})", self.username, self.active))
	}
}
```

- 사용법

```rust
let matthew = User::new();
```
