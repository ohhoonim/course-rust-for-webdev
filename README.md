# Rust Course for web developers

# Rust 초급 ✔️

## 개발환경 구성

윈도우용 프로그램을 개발하는 것이 아니라면, 윈도우의 WSL, Linux, Macos에서 개발하는 것을 권장합니다.

### Rust 설치

- rustup.rs
- 페이지에 있는 명령어를 카피하여 터미널에서 실행
- rustup을 사용하면 rustc와 cargo를 설치해준다.
- rustc: 컴파일러
- cargo: rust 빌드 도구

### 설치확인

```bash
$ rustc --version
$ cargo --version
```

### rustup 업데이트

```bash
$ rustup update
```

### IDE

처음하시는 분이라면 VSCode로 시작하는 것을 권장합니다.

- RustRover: jetbrains 에서 만든 Rust 전용 IDE
    - 다운로드: https://www.jetbrains.com/rust/
- Visual Studio Code: Microsoft에서 만든 범용 IDE
    - 다운로드: https://code.visualstudio.com/docs/languages/rust
    - rust-analyzer 확장 프로그램을 추가해준다.

## Cargo 빌드 도구로 프로젝트 생성하기

### Cargo로 새 프로젝트 만들기

```bash
$ cargo new [프로젝트명]
```

- `[프로젝트명]` 부분은 폴더명이 된다.
- 명령어를실행하면  `[프로젝트명]` 으로된 폴더가 생성된다.

### VSCode에서 프로젝트 열기

```bash
$ cd [프로젝트명]
$ code .
```

### 프로젝트 주요 파일 해설

- main.rs
    
    ```rust
    fn main() {
        println!("Hello, world!");
    }
    ```
    
    - `println!` 러스트에서 느낌표(!) 가 붙어있는 것을 ‘매크로’라고 한다.
    - `fn main()` 함수: 프로그램이 실행되는 시작점
    
    <aside>
    💡
    
    **매크로**
    
    ‘코드를 만들어주는 코드’ 정도로 이해하자. `println!` 매크로를 사용하지 않을 경우 다음과 같은 추가 코딩이 필요하다.
    
    ```rust
    use std::io::{self, Write};
    
    fn main() -> io::Result<()> {
        // 1. 표준 출력(stdout) 핸들을 가져옵니다.
        let mut stdout = io::stdout();
    
        // 2. 문자열을 바이트 슬라이스(b"...") 형태로 변환하여 직접 씁니다.
        // 매크로와 달리 줄바꿈 문자(\n)를 직접 넣어주어야 합니다.
        stdout.write_all(b"Hello, world!\n")?;
    
        // 3. 버퍼를 비워 화면에 즉시 출력되도록 합니다.
        stdout.flush()?;
    
        Ok(())
    }
    ```
    
    </aside>
    
- Cargo.toml
    
    ```toml
    [package]
    name = "course-rust-for-webdev"
    version = "0.1.0"
    edition = "2024"
    
    [dependencies]
    
    ```
    
    - `[package]`: 프로젝트 메타데이터를 정의. 패키지명, 버전, 러스트 에디션
    - `[dependencies]`: 외부 크레이트(라이브러기)를 지정
    - 그 외 빌드 프로파일 설정과 워크스페이스를 관리할 수 있다.
    - (참고) Cargo.lock 파일은 실제 다운로드된 의존성의 정확한 버전을 고정하고 기록한다.
    
    <aside>
    💡
    
    **크레이트**
    
    rust에서는 ‘라이브러리’라는 용어 대신 ‘크레이트’라는 용어를 사용한다. 
    
    </aside>
    

### 프로그램 실행하기

- IDE에서 실행하기
    
    main.rs 파일의 `fn main()`  위에 ‘▶ Run’ 부분을 클릭한다.
    
- 터미널에서 실행하기
    
    ```bash
    $ cargo run 
    ```
    
- 실행이 안될때 윈도우에서 추가 조치방법
    
    Visual Studio가 설치되어 있지 않은 경우(Visual Studio Code 아님), rustup-init이 사전 요구 사항을 자동으로 설치하지만 모든 사용자에게 적합하지 않을 수 있는 Visual Studio Community 에디션을 설치한다. 개인, 학계 및 오픈소스 사용에는 무료이지만, 회사내 조직에 속한 경우 어떤 에디션이 자신에게 적합한지 확인해야한다.
    
    https://rust-lang.github.io/rustup/installation/windows-msvc.html 을 참고하십시오. 
    

### (참고) **Cargo의 주요 표준 타겟 디렉토리**

<aside>
💡

**Cargo의 주요 표준 타겟 디렉토리**

Cargo는 코드의 목적에 따라 체계적으로 분리하여 사용하도록 표준구조를 지원한다.

Cargo에 대한 자세한 내용은 https://doc.rust-lang.org/cargo/index.html 에서 찾아볼 수 있다.

- `src/main.rs`: 메인 실행 프로그램 (cargo run)
- `src/lib.rs`: 라이브러리 크레이트. 다른 프로그램에서 사용할 수 있도록 모듈이나 함수를 묶어준다.
- `src/bin/`: 보조 실행 파일. main() 함수를 사용할 수 있다. (cargo run —bin 파일이름)
- `examples/`: 사용법 예제 코드들. main() 함수를 사용할 수 있다.( cargo run —example 파일이름)
- `tests/`: 이 폴더의 소스들은 통합테스트 파일로 인식한다. (cargo test)
- `benches/`: 벤치마크 코드를 작성하는 폴더이다. (cargo bench)

</aside>

## 변수와 가변성

러스트(Rust)에서 변수(Variable)와 가변성(Mutability)은 메모리 안전성을 보장하기 위한 핵심 철학 중 하나이다. Rust에서 변수는 기본적으로 값을 변경할 수 없는 '불변(Immutable)'이다.

### 1. 불변 변수 (Immutable Variables)

러스트에서 `let` 키워드로 변수를 선언하면, 기본적으로 그 값은 **변경할 수 없다(Read-only)**. 컴파일러가 값이 변하지 않는다는 것을 보장하므로, 멀티스레드 환경 등에서 안전하게 코드를 최적화하고 버그를 예방할 수 있다.

```rust
fn main() {
    let x = 5;
    // x = 6; // 에러
}
```

### 2. 가변 변수 (Mutable Variables)

만약 변수의 값을 나중에 바꾸고 싶다면, 변수를 선언할 때 `mut` 키워드를 명시하여 **가변 변수**로 만들어야 한다.

```rust
fn main() {
    let mut x = 5; 
    println!("초기값: {}", x);

    x = 6; // 정상 작동!
    println!("변경된 값: {}", x);
}
```

### 새도잉(Shadowing)

기존 변수의 이름을 재사용하여 새로운 변수를 선언할 수 있다. 이를 **섀도잉(Shadowing)** 이라고 한다. 이름만 같을 뿐 새로운 변수를 만드는 것이니 가변 변수와 구분하자.  

```rust
fn main() {
    let x = 5;
    let x = x + 1; // 기존 x를 가리키는 새로운 x 선언 (값은 6이 됨)

}
```

### 상수(const)

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

## 데이터 타입(data type)

변수를 선언할 때 타입을 적어주지 않더라도 rust 가 알아서 추론해주지만 데이터타입은 명시적으로 적어주자. 변수 타입은 ‘변수명:타입’ 형태로 적어준다.

### Scalar Types

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

<aside>
💡

rust에서는 ‘is_active’와 같이 스네이크 스타일을 사용하는 것이 네이밍 컨벤션(명명 규칙)이다.

</aside>

### Compound Types

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
    
- (참고)
    
    <aside>
    💡
    
    `&str` (string slice) 
    
    “Hello”처럼 문자열 리터럴을 사용할 때 만들어지는 타입이다.
    
    읽기 전용이며 ``static` 라이프사이클을 가진다. 우선은 이 정도만 알고 있자.
    
    </aside>
    
    <aside>
    💡
    
    **`{}` vs `{:?}`의 차이**
    
    현시점에서는 이런것도 있다 정도로만 기억하자
    
    - **`{}` (`Display` 트레이트)**
        - **용도**: 일반 사용자에게 보여주기 위한  **출력**
        - **대상**: 문자열, 숫자 등 기본 타입처럼 "어떻게 화면에 보여야 할지"가 명확한 타입들에만 기본 구현되어 있다.(scalar 타입들)
        - **특징**: 직접 만든 구조체(Struct)나 벡터(Vec) 등에 그냥 `{}`를 쓰면 컴파일 에러가 발생
    - **`{:?}` (`Debug` 트레이트)**
        - **용도**: 개발자가 디버깅을 목적으로 **데이터의 내부 구조나 상태를 확인**하기 위한 출력
        - **대상**: `Vec`, `HashMap`, 직접 만든 `Struct`나 `Enum` 등 복잡한 데이터 구조를 화면에 출력해보고 싶을때 사용
        - **특징**: `#[derive(Debug)]`라는 매크로만 붙여주면, 컴파일러가 자동으로 구조체의 내용을 출력할 수 있는 코드를 만들어 준다.
    </aside>
    

## 함수(Functions)

- `fn` 키워드 사용
- 타입명시 필수
- 반환타입: `->` 기호뒤에 적는다. 반환 값이 없다면 기호와 반환 타입을 안적는다. 이는 빈 튜플(`()`)을 반환하는 것과 같다.
- (중요) 함수 마지막줄에 세미콜론(;)을 붙이지 않으면 해당 값이 반환 값이다.(암시적 변환). 중간에 `return` 키워드 사용 가능하다.

```rust
// 함수 정의
fn add(x: i32, y: i32) -> i32 {
    x + y // 세미콜론이 없으므로 x + y의 결과가 자동으로 반환됩니다.
}

fn main() {
    // 함수 사용
    let result = add(5, 3);
    
    println!("결과: {}", result); // 출력: 결과: 8
}
```

## 제어 흐름(Control Flow)

### 조건 if / else

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

### 반복 loop, while, for

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

### 패턴매칭 match

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

## 소유권(Ownership)

소유권(ownership) 메모리 안정성 보장을 위한 러스트가 고안한 강력한 규칙이다.

### 원칙

- 모든값은 소유자가 있다.
- 소유자는 단 하나만 존재한다.
- 소유자가 범위를 벗어나면 Drop된다.

### 기본 동작 확인

```rust
let cart = String::from("my cart");
let other_cart = cart; // other의 소유권이 other_cart로 옮겨진다.

println!("{}", cart); // 이 카트는 비워졌으므로 꺼낼 물건이 없다. error!!
println!("{}", other_cart);
```

cart를 보고 똑같은 물건을 other_car에 별개로 담으면 된다. (서로 독립적이다.)

```rust
let cart = String::from("my cart");
let other_cart = cart.clone(); // 복제한다.

println!("{}", cart); 
println!("{}", other_cart);
```

## Borrow와 References

### move: 소유권 이동

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

### borrow: 소유권 빌리기(메모리 주소만 잠깐 사용하기)

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

<aside>
💡

usize

부호가 없는 정수( 0 이상). 32비트 시스템에서는 32비트(4바이트) 크기를 가지며, 64비트 시스템에서는 64비트(8바이트) 크기이다. 

- 배열이다 벡터의 인덱스는 무조건 usize이다.
- 컬렉션의 길이(len())나 크기(capacity())는 usize이다.
- usize는 메모리에 담을 수 있는 최대 개수이다. (포인터 크기와 일치)
</aside>

### references: 참조

- **`&` (불변 참조, Immutable Reference)**
    - **용도**: 데이터를 읽기만 할 때 사용(기본값)
    - **특징**: 원본 데이터를 수정할 수 없지만, **동시에 여러 개**의 불변 참조를 만드는 것 허용
- **`&mut` (가변 참조, Mutable Reference)**
    - **용도**: 빌려온 데이터를 수정(변경)해야 할 때 사용
    - **특징**: 데이터를 직접 바꿀 수 있기 때문에, 러스트에서는 데이터 경쟁(Data Race)을 막기 위해 **아주 엄격한 규칙**을 적용
    
    <aside>
    💡
    
    가변참조 사용시 주의 사항
    
    - 이미 만들어진 &mut가 존재하면 추가적인 참조자를 만들 수 없다
    - **불변 참조(`&`)들이 이미 여러 개 존재**하고 있다면, 그 데이터의 가변 참조(`&mut`)를 만들 수 없다 (읽고 있는데 갑자기 고치면 안 되기 때문)
    </aside>
    
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
    

## 구조체(Stucts)

러스트에서 관련 속성들을 하나로 묶어 사용자 정의 타입을 만드는 기능. 타 언어의 class와 유사하지만 상속(inheritance)이 없는 것이 특징이다. 

### 필드 구조체

가장 일반적인 형태. 필드에 이름을 부여한다.

```rust
#[derive(Debug)] // 디버그 프린트 기능을 자동으로 만들어준다.
struct User {
	username: String,
	email: String,
	active: bool,
}
```

### 튜플 구조체

필드 명을 부여하지 않고 타입 순서만 지정하는 구조체

```rust
struct Color(i32, i32, i32); // RGB 색상
```

### 유닛 유사 구조체

필드가 없는 구조체, 트레이트를 구현할 때만 사용하려는 경우 주로 사용

```rust
struct Empty;
```

### 구조체 기본 사용법

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

### 구조체에 메서드 추가하기

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

<aside>
💡

**self vs Self**

self: 인스턴스를 의미한다.

Self: 타입을 의미한다. 주로 리턴 타입이 자기 자신일때 사용한다.

</aside>

### 구조체에 연관 함수 추가하기

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

## 열거형(Enums)

### 기본 사용법

```rust
enum Direction {
	Up, 
	Down, 
	Left, 
	Right,
}

fn main() {
	let player_direction = Direction::Up;
}
```

### 데이터가 포함된 Enum

```rust
enum Message {
	Quit,
	Move {x: i32, y: i32},
	Write(String),
	ChangeColor(i32, i32, i32), 
}

fn main() {
	let msg1 = Message::Write(Sring::from("bye!");
	let msg2 = Message::Move {x: 10, y: 20},
}
```

### Enum에 대한 메서드 구현과 패턴매칭

structs와 마찬가지고 impl 키워드를 사용한다.

```rust
enum Status {
	Active(i32),
	Inactive,
}

impl Ststuss {
	fn check(&self) {
      match self {
          Status::Active(id) => println!("활성화 상태 (ID: {})", id),
          Status::Inactive => println!("비활성화 상태"),
      }
  }
}

fn main() {
	let user_status = Status::Active(42);
	user_status.check();
}
```

## Option과 Result

### (참고) Generic

<aside>
💡

Generic

코드에서 Option<String> 과 같이 꺽쇠로 표현하는 것을 ‘Generic’이라고 한다. Trait & Generic 에서 설명할 에정.

</aside>

### Option

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

### Result

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

## Collections: Vec, String, HashMap

### Rust 표준라이브러리와 용어 정리

rust에서 제공하는 표준라이브러리를 std라고 표현한다. 그 중 데이터 구조들은 `std::collections` 모듈에 모여있다.  

- **패키지 (Package)**: 하나 이상의 크레이트를 빌드, 테스트, 공유할 수 있도록 묶어주는 단위입니다. `Cargo.toml` 파일 하나가 하나의 패키지를 관리한다.
- **크레이트 (Crate)**: 컴파일하는 최소 단위. 라이브러리 크레이트나 실행 가능한 바이너리 크레이트가 될 수 있다. (`std` 는 표준 라이브러리 크레이트이다).
- **모듈 (Module)**: 크레이트 내부에서 코드를 논리적으로 그룹화하고 공개 범위를 제어하는 단위입니다 (`mod` 키워드로 선언)

### collection 분류

- 시퀀스(Sequences)
    - 순서가 있는 데이터 모음. 인덱스를 사용
    - Vec, VecDeque, LinkedList 등
- 맵(Maps)
    - 키와 값을 동시에 저장하는 방식
    - HashMap, BTreeMap 등
- 셋(Sets)
    - 순서를 따지지 않는 데이터 모음. 중복을 허용하지 않는다.
    - HashSet, BTreeSet 등

### Vec

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

### String

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

### HashMap

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

## Trait & Generic

### Generic

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
    - **`PartialOrd`**: 값이 서로 크다 작다를 비교(`>`)할 수 있는 기능(트레이트)이 반드시 구현되어 있어야 함을 의미.
    - **`Copy`**: 대입 연산 시 값이 복사되도록 하여, `largest = item;`처럼 변수를 할당할 때 소유권이 이동(Move)하지 않고 복사(Copy)될 수 있는 타입이어야 한다는 뜻
    - **`+` (플러스 기호)**: 여러 트레이트 제약을 동시에 걸어줄 때 사용하는 **And(그리고)** 연산자 (즉, `PartialOrd`와 `Copy`를 **둘 다** 구현한 타입만 허용).

### Trait

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
    

### STD 주요 트레이트

- 1. 마커 및 복제 트레이트 (`std::marker` / `std::clone`)
    - **`Copy`**: 비트 단위 복사가 가능하여, 대입 시 소유권 이동 대신 복사가 일어나는 타입
    - **`Clone`**: 명시적인 복제(`clone()`) 메서드를 제공하여 깊은 복사가 가능한 타입
    - **`Sized`**: 컴파일 타임에 크기를 명확히 알 수 있는 타입 (대부분의 타입이 기본 적용)
    - **`Send` / `Sync`**: 스레드 간 안전한 소유권 이동(`Send`) 및 참조 공유(`Sync`)가 가능한 타입
- 2. 비교 트레이트 (`std::cmp`)
    - **`PartialEq`**: 동등성 비교(`==`, `!=`)가 가능한 타입
    - **`Eq`**: 완전한 동등성 비교가 가능한 타입 (자기 자신과 항상 같음 보장)
    - **`PartialOrd`**: 크기 비교(`<`, `>`, `<=`, `>=`)가 가능한 타입
    - **`Ord`**: 완전한 대소 비교가 가능하여 정렬(Sorting)이 가능한 타입
- 3. 변환 및 포맷 트레이트 (`std::convert` / `std::fmt`)
    - **`From` / `Into`**: 다른 타입으로부터 자기 자신을 생성하거나 변환하는 트레이트
    - **`Display`**: 사용자 친화적인 문자열 표현을 제공 (`{}` 포맷터 사용)
    - **`Debug`**: 개발자용 디버깅 문자열 표현을 제공 (`{:?}` 포맷터 사용)
- 4. 연산자 및 메모리 관리 트레이트 (`std::ops`)
    - **`Add`, `Sub`, `Mul`, `Div`**: 산술 연산자(`+`, , , `/`)를 오버로딩할 수 있는 트레이트
    - **`Deref`**: 참조자 역참조( 및 자동 역참조) 동작을 커스텀하는 트레이트
    - **`Drop`**: 값이 스코프를 벗어날 때 실행할 정리 작업(소멸자)을 정의하는 트레이트
    - **`Default`**: 타입의 기본값(`Default::default()`)을 제공하는 트레이트
- 5. 이터레이터 트레이트 (`std::iter`)
    - **`Iterator`**: 요소를 순차적으로 순회하며 처리할 수 있는 반복자 동작을 정의하는 트레이트

---

# Rust 중급 ✔️

## Modules, Cates and Project Organization ✔️

### 목표

모듈, 크레이트, 워크스페이스를 이해하고 프로젝트를 적절히 구성한다.

### 상황: 모든 코드를 main() 함수에 넣을 수는 없다. 프로젝트내 파일들을 목적에 맞게 분리한다.

1. ‘tactics-rust-warehouse’ 명으로 프로젝트를 생성하고 main.rc에 ‘Product’ stuct를 작성하기
2. struct 를 ‘model.rs’ 파일로 분리하기(모듈화) 
3. workspace를 사용하여  크래이트를 ‘core’와 ‘cli’ 로 나누기
    1. core: 라이브러리
    2. cli: 바이너리, core를 dependency 한다.
4. model.rs는 core로 이동시키고, main.rs의 소스를 cli로 옮기기

### 힌트

- 네이밍 컨벤션: 프로젝트명(패키지명)은 하이픈(-)을 사용하고 변수명은 언더스코어(_)를 사용한다. 코드에서 use 사용시 하이픈을 언더스코어로 rust가 자동 변환해준다.
- 프로젝트 생성
    
    ```bash
    $ cargo new tactics-rust-warehouse
    $ cd tactics-rust-warehouse
    $ code .
    ```
    
- dependences
    
    ```bash
    $ cargo add chrono serde --features serde/derive uuid --features uuid/v4
    
    $ cargo add uuid --features serde,v4
    $ cargo add chrono --features serde
    ```
    
    또는 Cargo.toml 을 직접수정
    
    ```toml
    [dependencies]
    chrono = { version = "0.4.45", features = ["serde"] }
    serde = { version = "1.0.229", features = ["derive"] }
    uuid = { version = "1.28.0", features = ["serde", "v4"] }
    ```
    
- structs
    
    ```rust
    
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Dimensions {
        pub length: f64,
        pub width: f64,
        pub height: f64,
    }
    
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Product {
        pub id: Uuid,                  // 상품 고유 식별자 (UUID)
        pub sku: String,               // 재고 관리 코드 (Stock Keeping Unit)
        pub name: String,              // 상품명
        pub description: Option<String>, // 상품 상세 설명
        pub category: String,          // 상품 카테고리
        pub unit_price: u32,           // 단가 (원화 기준 정수형 권장)
        pub weight: f64,               // 중량 (kg 단위)
        pub dimensions: Dimensions,    // 치수 (가로, 세로, 높이)
        pub stock_quantity: u32,       // 현재 창고 재고 수량
        pub reorder_level: u32,        // 안전재고 기준선 (부족 시 발주 알림)
        pub location: Option<String>,  // 창고 내 보관 위치 (예: Zone-A, Rack-03)
        pub created_at: DateTime<Utc>, // 등록 일시
        pub updated_at: DateTime<Utc>, // 최종 수정 일시
    }
    ```
    
- workspace 로 변경한 뒤 Cargo.toml
    
    ```toml
    ############################# root cargo
    [workspace]
    members = ["core", "cli"]
    
    [workspace.dependencies]
    chrono = { version = "0.4.45", features = ["serde"] }
    serde = { version = "1.0.229", features = ["derive"] }
    uuid = { version = "1.28.0", features = ["serde", "v4"] }
    
    ############################ core
    [package]
    name = "core"
    version = "0.1.0"
    edition = "2024"
    
    [dependencies]
    serde.workspace = true
    uuid.workspace = true
    chrono.workspace = true
    
    ########################## cli
    [package]
    name = "cli"
    version = "0.1.0"
    edition = "2024"
    
    [dependencies]
    core = { path = "../core" }
    uuid.workspace = true
    chrono.workspace= true
        
    ```
    

### 결과

!image.png

### 과제

- Crate에서 커스텀 feature를 제공하는 방법을 조사하시오.
- cargo build 시 crate 단위로 빌드하려면 어떤 옵션을 사용해야하느가?

## Iterators & Closures✔️

### 목표

- **클로저(Closure)**
    - 클로저의 구문(Syntax)과 타입 추론 방식을 이해합니다.
    - 환경 캡처 방식에 따른 세 가지 트레이트(`Fn`, `FnMut`, `FnOnce`)의 차이와 소유권 이동 규칙을 파악합니다.
- **이터레이터(Iterator)**
    - 이터레이터의 지연 평가(Lazy evaluation) 특성을 이해합니다.
    - `iter()`, `iter_mut()`, `into_iter()`의 차이를 구분하고 상황에 맞게 적용합니다.
    - `map`, `filter`, `fold`, `collect` 등 다양한 어댑터와 소비자 메서드를 조합하여 복잡한 데이터를 선언적으로 처리합니다.

### 상황: 쇼핑몰 주문 로그 분석

일련의 주문 데이터가 들어 있는 벡터를 처리하는 프로그램을 작성합니다. 클로저와 이터레이터를 조합하여 데이터를 필터링하고 통계를 내는 작업을 수행

- **배송 완료된 주문만 필터링 (`filter`)**: `is_delivered`가 `true`인 주문들만 골라냅니다.
- **총 주문 금액 계산 (`map`과 `fold` 또는 `sum`)**: 필터링된 각 주문의 가격 곱하기 수량(`price * quantity`)을 계산한 뒤, 총합을 구합니다.
- **고가 상품 리스트 추출 (`filter`와 `map` 및 `collect`)**: 배송 여부와 상관없이 총 가격(`price * quantity`)이 100,000원을 초과하는 상품들의 이름(`item_name`)만 모아서 새로운 `Vec<String>`으로 수집합니다. 이때 외부 변수(기준 금액 100,000원)를 캡처하는 **클로저**를 사용해 보세요.

### 힌트

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
        - **기본 형태**: `|x, y| x + y`
        - **타입 명시**: 기본적으로 타입은 자동으로 추론되며, 필요하다면  타입을 적어도 된다.(예: `|x: i32, y: i32| -> i32 { x + y }`)
        - **블록 본문**: 코드가 한 줄이면 중괄호 `{}`를 생략할 있다.
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
    

### 결과

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
    

### 과제

- 과제 1: `iter_mut()`를 이용한 일괄 할인 적용
    - **목표**: 가변 참조를 다루는 이터레이터와 클로저를 사용하여 데이터 직접 수정하기
    - **내용**: `.iter_mut()`를 사용해 아직 배송되지 않은(`is_delivered == false`) 모든 주문의 가격(`price`)을 10% 할인된 값(`price * 90 / 100`)으로 수정하고, 전체 주문 목록을 출력하는 코드를 작성하세요.
- 과제 2: `into_iter()`와 소유권을 활용한 데이터 변환
    - **목표**: 소유권을 이전하는 이터레이터를 이해하고 새로운 구조체 형태로 가공하기
    - **내용**: 기존의 `orders` 벡터를 `.into_iter()`로 소비하면서, 각 주문을 아래의 요약 구조체로 변환(`map` 및 `collect`)하는 코드를 작성하세요.
        
        ```rust
        struct OrderSummary {
            id: u32,
            description: String, // 예: "Keyboard (2개)" 형태로 포맷팅
        }
        ```
        
- 과제 3: `any`와 `max_by_key`를 활용한 조건 검사와 집계
    - **목표**: 단일 값이나 조건 만족 여부를 반환하는 소비자(Consumer) 메서드 사용하기
    - **내용**: 클로저를 활용해 다음 동작을 수행하는 코드를 각각 작성하세요.
        1. `.any()`를 사용하여 수량이 5개 이상인 주문이 하나라도 존재하는지 확인(`bool` 반환)
        2. `.max_by_key()`를 사용하여 총 주문 금액(`price * quantity`)이 가장 큰 주문 객체를 찾아내기

### 과제 풀이

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

## Lifetimes✔️

### 목표

- **댕글링 참조(Dangling Reference) 방지** 원리 이해: 러스트 컴파일러가 메모리 안정성을 보장하기 위해 참조의 유효 기간을 어떻게 추적하는지 파악
- **명시적 수명 주석**(`'a`) 문법 : 함수나 구조체에서 여러 참조자가 얽혀 있을 때 컴파일러에게 관계를 명시해 주는 표기법을 사용한다.
- **수명 생략 규칙**(Lifetime Elision Rules) 이해: 개발자가 수동으로 적지 않아도 컴파일러가 자동으로 수명을 유추해 주는 규칙과 그 한계를 구분한다.
- **구조체와 참조의 관계**: 구조체가 다른 데이터의 참조를 필드로 가질 때 수명 주석을 설계하는 방법

### 상황: 가장 긴 상품 이름을 가진 주문 찾기

입력으로 들어온 두 개의 주문(`&Order`) 중 상품 이름(`item_name`)의 길이가 더 긴 주문의 참조를 반환하는 함수 `get_longer_order(order1: &Order, order2: &Order) -> &Order`를 작성

- 단계 1: 컴파일 에러 직접 마주하기
    
    아래 코드를 그대로 작성하고 `cargo check`를 실행해 컴파일 에러 메시지를 확인
    
    ```rust
    fn get_longer_order(order1: &Order, order2: &Order) -> &Order {
        if order1.item_name.len() > order2.item_name.len() {
            order1
        } else {
            order2
        }
    }
    ```
    
- 단계 2: 수명 주석(`'a`) 추가하여 컴파일 통과시키기
    
    ```rust
    // 힌트 구조: fn get_longer_order<'a>(...) -> &'a Order
    ```
    
- 단계 3: 구조체와 수명 주석 결합하기 (심화)
    
    ```rust
    struct FeaturedOrder<'a> {
        // 여기에 Order의 참조를 담는 필드를 정의
    }
    ```
    

### 힌트

- Order 구조체
    
    ```rust
    struct Order {
        id: u32,
        item_name: String,
        price: u32,
        quantity: u32,
        is_delivered: bool,
    }
    ```
    
- 댕글링 참조 방지 원리
    
    댕글링 참조(Dangling Reference)란 이미 메모리에서 해제되었거나 소멸한 데이터의 주소를 가리키고 있는 포인터나 참조. 비정상 종료나 심각한 보안 취약점을 유발한다.
    
    러스트는 Ownership, Scope, Borrow chcker, Lifetimes 등으로 컴파일 단계에서 댕글링 참조를 방지한다. 
    
- 수명 생략 규칙
    
    입력이 하나뿐이거나 메서드일 때 알아서 수명을 채워주며, 입력이 여러개인 경우 등 관계가 모호 할 때는 명식적으로 수명 주석을 사용하면 된다. 코드에서 누가 오래 살아 남아야 하는가의 결정은 개발자의 몫이다. 
    
    ‘`a’ 는 그냥 ‘`a’ 일 뿐이다. ‘`b’ 라고 적어도 된다.
    
    - 1. 입력 수명 규칙 (Input Lifetime Rule)
        
        함수의 각 매개변수 중 참조자인 것들은 **각각 고유한 수명 주석**을 하나씩 부여받습니다.
        
        - 예: `fn foo(x: &i32, y: &i32)`는 컴파일러가 내부적으로 `fn foo<'a, 'b>(x: &'a i32, y: &'b i32)`로 인식합니다.
    - 2. 단일 입력 수명 규칙 (Single Input Lifetime Rule)
        
        만약 입력 매개변수가 **정확히 단 하나**뿐이라면, 그 하나의 수명이 **모든 반환값의 수명**으로 그대로 할당됩니다.
        
        - 예: `fn foo(s: &str) -> &str`는 컴파일러가 자동으로 `fn foo<'a>(s: &'a str) -> &'a str`로 해석합니다. 입력된 참조의 수명이 그대로 반환값까지 이어짐을 보장합니다.
    - 3. 메서드 수명 규칙 (Method Self Lifetime Rule)
        
        메서드의 경우, 매개변수에 `&self`나 `&mut self`가 포함되어 있다면 **`self`의 수명**이 모든 반환값의 수명으로 할당됩니다.
        

### 결과

- 단계1: 컴파일 에러분석
    - `missing lifetime specifier`
    - 반환하는 참조가 `order1`과 `order2` 중 어느 것과 수명이 일치하는지 컴파일러가 알 수 없기 때문입니다. 함수가 끝난 뒤에도 반환된 참조가 안전하게 유효한지 보장하기 위해 수명 관계를 명시해야 한다.
    
    ```rust
    // 에러가 발생하는 코드
    fn get_longer_order(order1: &Order, order2: &Order) -> &Order {
        if order1.item_name.len() > order2.item_name.len() {
            order1
        } else {
            order2
        }
    }
    ```
    
- 단계2: 명시적 수명 주석(`'a`) 추가
    
    입력 인자와 반환값의 수명이 같거나 최소한 `'a` 기간 동안은 유효함을 컴파일러에게 알려준다.
    
    ```rust
    fn get_longer_order<'a>(order1: &'a Order, order2: &'a Order) -> &'a Order {
        if order1.item_name.len() > order2.item_name.len() {
            order1
        } else {
            order2
        }
    }
    ```
    
- 단계3: 구조체와 수명 주석 결합
    
    구조체가 참조자를 필드로 가질 때는, 그 참조자가 구조체보다 먼저 소멸되지 않도록 구조체 정의 자체에 수명 주석을 명시해 준다.
    
    ```rust
    struct FeaturedOrder<'a> {
        order: &'a Order,
        highlight_reason: String,
    }
    ```
    
- 전체 코드
    
    ```rust
    struct Order {
        id: u32,
        item_name: String,
        price: u32,
        quantity: u32,
        is_delivered: bool,
    }
    
    // 수명 주석을 포함한 비교 함수
    fn get_longer_order<'a>(order1: &'a Order, order2: &'a Order) -> &'a Order {
        if order1.item_name.len() > order2.item_name.len() {
            order1
        } else {
            order2
        }
    }
    
    // 참조자를 필드로 가지는 구조체 (수명 주석 필수)
    struct FeaturedOrder<'a> {
        order: &'a Order,
        highlight_reason: String,
    }
    
    fn main() {
        let order_a = Order {
            id: 1,
            item_name: String::from("Wireless Mouse"),
            price: 30_000,
            quantity: 1,
            is_delivered: true,
        };
    
        let order_b = Order {
            id: 2,
            item_name: String::from("Mechanical Keyboard"),
            price: 120_000,
            quantity: 1,
            is_delivered: false,
        };
    
        // 더 긴 이름을 가진 주문의 참조를 가져옵니다.
        let longer = get_longer_order(&order_a, &order_b);
        println!("더 긴 상품 이름: {}", longer.item_name);
    
        // 구조체에 참조를 안전하게 담습니다.
        let featured = FeaturedOrder {
            order: longer,
            highlight_reason: String::from("가장 긴 상품명 보유"),
        };
    
        println!("특집 주문 선정 이유: {}", featured.highlight_reason);
    }
    ```
    

### 과제

- 상황 설정
    
    기존의 `Order` 구조체 데이터가 있을 때, 원본 데이터를 직접 복사하거나 소유권을 빼앗아 오지 않고 **참조(`&Order`)를 안전하게 담아두는 전용 뷰어 구조체**를 만들려고 합니다.
    
- 요구사항
    1. **함수 작성 (`find_first_express_order`)**
        - 주문 슬라이스(`&[Order]`)를 인자로 받아, 가격이 50,000원을 넘는 **첫 번째 주문의 참조**를 반환하는 함수를 작성하세요.
        - 반환 타입은 `Option<&Order>` 형태여야 합니다. (조건에 맞는 주문이 없을 수도 있으므로)
        - *힌트*: 입력이 하나이므로 수명 생략 규칙이 적용되지만, 명시적으로 `'a`를 붙여 시그니처를 직접 완성해 보세요.
    2. **구조체 작성 (`OrderViewer`)**
        - 원본 주문의 참조를 필드로 품고 있는 구조체 `OrderViewer<'a>`를 정의하세요.
        - 필드 구성:
            - `target_order`: `&'a Order` (찾아낸 주문의 참조)
            - `viewer_note`: `String` (검토 메모)
        - 구조체 정의 시 수명 주석(`'a`)을 올바르게 배치해야 합니다.
    3. **`main` 함수 완성**
        - `Order` 인스턴스들이 담긴 벡터를 만들고, `find_first_express_order`를 호출하여 얻은 참조를 `OrderViewer`에 안전하게 담아 출력해 보세요.
- 템플릿 코드
    
    ```rust
    struct Order {
        id: u32,
        item_name: String,
        price: u32,
        quantity: u32,
        is_delivered: bool,
    }
    
    // 1. 수명 주석을 명시한 검색 함수 완성하기
    fn find_first_express_order(orders: &[Order]) -> Option<&Order> {
        // 여기에 코드를 작성하세요 (가격이 50000 초과인 첫 번째 주문 찾기)
        todo!()
    }
    
    // 2. 참조를 필드로 가지는 구조체 정의하기
    struct OrderViewer {
        // 여기에 필드를 채워보세요
    }
    
    fn main() {
        let orders = vec![
            Order { id: 1, item_name: String::from("Pen"), price: 2_000, quantity: 5, is_delivered: true },
            Order { id: 2, item_name: String::from("Keyboard"), price: 75_000, quantity: 1, is_delivered: false },
        ];
    
        // 3. 함수를 호출하고 OrderViewer 생성하기
        // let target = ...;
        // let viewer = ...;
        
        // println!("뷰어 대상 상품: {}", viewer.target_order.item_name);
    }
    ```
    

### 과제 풀이

```rust
struct Order {
    id: u32,
    item_name: String,
    price: u32,
    quantity: u32,
    is_delivered: bool,
}

fn find_first_express_order<'a>(orders: &'a [Order]) -> Option<&'a Order> {
    orders.iter().find(|order| order.price > 50_000)
}

struct OrderViewer<'a> {
    target_order: &'a Order,
    viewer_note: String,
}

fn main() {
    let orders = vec![
        Order { id: 1, item_name: String::from("Pen"), price: 2_000, quantity: 5, is_delivered: true },
        Order { id: 2, item_name: String::from("Keyboard"), price: 75_000, quantity: 1, is_delivered: false },
    ];

    if let Some(target) = find_first_express_order(&orders) {
        let viewer = OrderViewer {
            target_order: target,
            viewer_note: String::from("고가 상품 검토 대상"),
        };
        
        println!("뷰어 대상 상품: {}", viewer.target_order.item_name);
        println!("검토 메모: {}", viewer.viewer_note);
        println!("상품 가격: {}원", viewer.target_order.price);
    } else {
        println!("조건에 맞는 주문이 없습니다.");
    }
}
```

## Smart Pointers: Box, Rc, RefCell✔️

### 목표

- **힙 메모리 할당과 크기 제어 (`Box<T>`)**: 컴파일 타임에 크기를 알 수 없는 **재귀적 타입**이나 **큰 데이터를** 힙에 안전하게 할당하고 관리하는 법
- **단일 스레드 공유 소유권 (`Rc<T>`)**: 하나의 데이터를 여러 곳에서 동시에 소유해야 할 때 참조 카운팅(Reference Counting)을 통해 안전하게 공유하는 법을 파악
- **내부 가변성(`RefCell<T>`)**: 대여 규칙(Borrowing Rules)을 컴파일 타임이 아닌 런타임에 검사하여, 불변 참조 상태에서도 내부 값을 변경할 수 있는 패턴 이해
- **스마트 포인터 조합 (`Rc<RefCell<T>>`)**: 실무에서 자주 쓰이는 **공유 + 가변 상태 조합의 패턴**

### 상황: 할인 정책과 이벤트 로그를 공유하는 주문 시스템

**요구사항**

- 1. 재귀적 번들 상품 구조 구현 (`Box<T>`)
    - **요구사항**: `Order` 구조체는 선택적으로 또 다른 하위 주문(`sub_order`)을 가질 수 있어야 합니다.
    - **제약 조건**: 러스트 컴파일러가 구조체의 크기를 컴파일 시점에 확정할 수 있도록, 하위 주문 필드는 반드시 힙에 할당되는 포인터 타입(`Box<Order>`)으로 감싸야 합니다.
- 2. 쿠폰 공유 소유권 관리 (`Rc<T>`)
    - **요구사항**: 여러 개의 독립적인 `Order` 인스턴스가 단 하나의 `DiscountCoupon` 객체를 중복 생성 없이 동시에 참조할 수 있어야 합니다.
    - **제약 조건**: 쿠폰 객체를 `Rc<T>`로 감싸고, 새로운 주문이 쿠폰을 참조할 때마다 `Rc::clone`을 호출하여 참조 카운트가 정상적으로 증가하는지 확인할 수 있어야 합니다.
- 3. 런타임 내부 가변성 적용 (`RefCell<T>`)
    - **요구사항**: 쿠폰을 공유하고 있는 여러 주문 중 어느 한쪽에서 결제가 완료되면, 공유된 쿠폰의 남은 사용 횟수(`remaining_uses`)를 프로그램 실행 중에 즉시 차감할 수 있어야 합니다.
    - **제약 조건**: 쿠폰 변수 자체가 불변(`let`)으로 선언되어 있더라도, `RefCell<T>`의 `.borrow_mut()` 메서드를 통해 런타임에 안전하게 가변 대여를 획득하여 값을 수정할 수 있어야 합니다.
- 4. 상태 동기화 검증
    - **요구사항**: 하나의 주문을 통해 쿠폰 사용 횟수가 변경되었을 때, 다른 주문이나 외부에서 동일한 쿠폰을 조회(`borrow()`)했을 때 변경된 값이 정상적으로 반영(공유)되어 있는지 출력(Validation)을 통해 확인할 수 있어야 합니다.

### 힌트

- structs: Order, DiscountCoupon, **‘???’** 이부분을 채우세요
    
    
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
    
    - **힌트**: 컴파일 타임에 크기가 불확실한 데이터를 **힙 메모리**로 밀어내고, 구조체 안에는 고정된 크기의 주소값(포인터)만 남기려면 `Box<T>`로 감싸야 합니다. 포인터의 크기는 고정되어 있으므로 컴파일러가 크기를 계산할 수 있게 됩니다.
- 상황 2: `Rc<T>`와 단일 스레드 공유 소유권 (할인 쿠폰)
    
    러스트의 기본 원칙은 하나의 데이터는 정확히 하나의 소유자만 가질 수 있다는 것입니다. 하지만 여러 주문서가 동일한 쿠폰 데이터를 복사 없이 동시에 참조해야 할 때가 있습니다.
    
    - **힌트**: 데이터를 여러 곳에서 안전하게 공유하되, 누군가 참조를 포기할 때마다 카운트를 세어 마지막 소유자가 사라질 때 메모리를 해제해 주는 `Rc<T>`(Reference Counted)를 사용해 보세요. 변수를 복사할 때 데이터 자체가 아니라 `Rc` 자체를 `.clone()`하면 참조 카운트만 올라갑니다.
- 상황 3: `RefCell<T>`와 내부 가변성 (쿠폰 수량 변경)
    
    `Rc<T>`를 사용하면 여러 곳에서 데이터를 공유할 수 있지만, 기본적으로 불변(Immutable) 상태로만 공유됩니다. 만약 공유받은 곳 중 한 곳에서 쿠폰의 남은 수량을 깎는 등의 변경을 가하려고 하면 컴파일 에러가 발생합니다.
    
    - **힌트**: 대여 규칙 검사를 컴파일 타임이 아니라 프로그램 실행 중(런타임)으로 미루어주는 `RefCell<T>`를 `Rc`내부에 중첩(`Rc<RefCell<T>>`)해서 사용해 보세요. `.borrow_mut()`를 통해 불변 참조 상태에서도 값을 안전하게 변경할 수 있습니다.
- ref 키워드
    
    러스트에서 `ref` 키워드는 **패턴 매칭 시 소유권을 이동(Move)시키는 대신, 참조자(`&`) 형태로 값을 빌려오고 싶을 때** 사용하는 키워드입니다.
    
    러스트의 기본 규칙에 따라 `match`나 `if let` 등으로 구조체나 열거형의 내부 값을 패턴 매칭하면, 기본적으로 값의 **소유권이 변수로 이동**합니다. 이 경우 원본 객체의 소유권이 깨지거나 복사할 수 없는 타입일 경우 컴파일 에러가 발생합니다.
    
    이때 `ref`를 사용하면 원본의 소유권을 그대로 둔 채, 안전하게 참조(`&`) 형태의 바인딩을 만들어 낼 수 있습니다.
    
    - 종류
        - **`ref`**: 불변 참조(`&T`)를 가져옵니다.
        - **`ref mut`**: 가변 참조(`&mut T`)를 가져옵니다. 내부 값을 수정해야 할 때 주로 사용합니다.
    
    최신 러스트에서는 굳이 패턴 내부에 `ref`를 적지 않고, 대입하려는 대상 앞에 `&`를 붙이는 방식을 더 권장하기도 합니다.
    
    ```rust
    // ref 키워드를 쓰는 방식
    if let Some(ref coupon_ref) = order1.coupon { ... }
    
    // 오른쪽에 &를 붙이는 현대적인 방식 (결과적으로 동일하게 참조를 얻음)
    if let Some(coupon_ref) = &order1.coupon { ... }
    ```
    

### 결과

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

### 과제

- 상황 설정
    
    온라인 쇼핑몰 시스템을 고도화하고 있습니다. 상품의 계층형 카테고리(상위/하위 카테고리)를 표현하기 위해 재귀적 구조가 필요하며, 여러 주문서가 **동일한 실시간 재고 객체**를 공유하면서 결제 시 수량이 동기화되어야 합니다.
    
- 요구사항
    1. **재귀적 카테고리 구조체 (`Box<T>`)**
        - 이름(`name: String`)과 선택적 하위 카테고리(`sub_category: Option<Box<Category>>`)를 필드로 가지는 `Category` 구조체를 정의하세요.
        - 상위 카테고리 안에 하위 카테고리를 중첩하여 생성할 수 있어야 합니다.
    2. **공유 재고 구조체 정의 (`Rc<RefCell<T>>`)**
        - 상품명(`item_name: String`)과 남은 수량(`stock_count: i32`)을 필드로 가지는 `Inventory` 구조체를 정의하세요.
    3. **주문 및 재고 차감 로직 구현**
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
    

### 과제 풀이

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

## Advanced Traits✔️

### 목표

트레이트(Trait)는 단순히 인터페이스 개념을 넘어 3가지 용법에 대해 이해하고 활용한다.

- 공통 인터페이스 정의 및 다형성 (Shared Interface & Polymorphism)
    - 제네릭의 트레이트 바운드(`T: Summary`)를 통해 컴파일 타임에 안전한 정적 다형성을 구현하거나, 트레이트 객체(`dyn Trait`)를 통해 런타임 동적 다형성을 구현할 수 있다.
- 확장 트레이트 (Extension Traits)
    - 특정 타입에 존재하지 않는 도메인 고유의 편의 메서드를 추가하여 코드의 가독성을 높이고 체이닝(Chaining) 패턴을 구현할 때 주로 사용
- 연산자 오버로딩 및 표준 동작 커스터마이징 (Operator Overloading & Standard Trait Integration)
    - `Add`, `Sub` 등의 트레이트를 구현하여 커스텀 타입끼리 수학 연산자(`+`, )를 사용할 수 있게 한다 (`연산자 오버로딩`).
    - `Display`, `Debug`를 구현해 `println!` 출력을 지원
    - `Drop`을 구현해 객체가 소멸할 때의 커스텀 정리 로직을 실행

### 상황

- 1. 공통 인터페이스 정의 및 다형성
    
    Order, Membership struct에 ‘describe()’ 공통 메서드를 정의한다.
    
- 2. 확장 트레이트
    
    문자열 슬라이스(&str)에 주문 번호 포맷팅(’[ORDER-ID]: {}’) 기능을 추가한다.
    
- 3. 연산자 오버로딩 및 표준 동작 커스터마이징
    
    Money struct를 만들고, std::ops::Add 트레이트를 오버라이드하여 연산자 ‘+’ 기능을 추가한다.
    

### 힌트

```rust
//1. 공통 인터페이스 정의 및 다형성 (Shared Interface & Polymorphism)

trait Describable {
    fn describe(&self) -> String;
}

struct Order {
    item_name: String,
    price: u32,
}

struct Membership {
    tier: String,
}

impl Describable for Order {
    fn describe(&self) -> String {
        format!("주문 상품: {}, 가격: {}원", self.item_name, self.price)
    }
}

impl Describable for Membership {
    fn describe(&self) -> String {
        format!("멤버십 등급: {}", self.tier)
    }
}

fn print_info<T: Describable>(item: &T) {
    println!("{}", item.describe());
}
```

```rust
// 2. 확장 트레이트 (Extension Traits)
// 표준 타입(str)이나 외부 라이브러리 타입을 직접 수정할 수 없을 때

// 문자열에 주문 번호 포맷팅 기능을 추가
trait OrderIdExt {
    fn to_order_tag(&self) -> String;
}

impl OrderIdExt for str {
    fn to_order_tag(&self) -> String {
        format!("[ORDER-ID]: {}", self)
    }
}
```

```rust

// 3. 연산자 오버로딩 및 표준 동작 커스터마이징 (Operator Overloading & Standard Integration)
use std::ops::Add;

#[derive(Debug)]
struct Money(f32);

impl Add for Money {
    type Output = Money;

    fn add(self, rhs: Self) -> Self::Output {
        Money(self.0 + rhs.0)
    }
}
```

### 결과

- 1. 공통 인터페이스 정의 및 다형성

```rust
fn shared_interface_polymorphism() {
    let order = Order {
        item_name: String::from("bag"),
        price: 100_000,
    };
    let membership = Membership {
        tier: String::from("Silver"),
    };

    print_info(&order);
    print_info(&membership);
}
```

- 2. 확장 트레이트

```rust
fn str_ext_to_order_id() {
    let raw_id = "A1B1C-2";
    println!("{}", raw_id.to_order_tag());
}
```

- 3. 연산자 오버로딩 및 표준 동작 커스터마이징

```rust
fn override_add_for_money() {
    let m1 = Money(100_000.12);
    let m2 = Money(1_900.10);

    let total = m1 + m2;
    println!("{:2}", total.0);
}
```

### 과제

트레이트의 3가지 용법을 직접 구현하고 체득할 수 있는 **이커머스 도메인 기반의 실습 과제**입니다.

- 과제 1: 공통 인터페이스와 다형성 (`Summary` 트레이트)
    - **목표**: 서로 다른 상품 타입에 공통 인터페이스를 부여하고, 제네릭 함수를 통해 다형적으로 처리하기.
    - **요구사항**:
        1. `PhysicalProduct`(실물 상품)와 `DigitalProduct`(디지털 상품) 구조체를 각각 정의하세요. (필드는 자율 구성, 예: 이름, 가격, 용량 등)
        2. 두 구조체가 공통으로 구현해야 하는 `Summary` 트레이트를 정의하세요. 이 트레이트는 상품의 요약 정보를 `String`으로 반환하는 `summarize(&self) -> String` 메서드를 가져야 합니다.
        3. 각 구조체에 `Summary` 트레이트를 구현하세요.
        4. `T: Summary` 제네릭 바운드를 사용하는 함수 `print_product_summary(item: &T)`를 작성하고, `main` 함수에서 두 상품을 인자로 전달해 각각 출력해 보세요.
- 과제 2: 확장 트레이트 (`PriceFormatter` 확장)
    - **목표**: 기본 원시 타입(`u32` 등)에 도메인 전용 포맷팅 메서드 부착하기.
    - **요구사항**:
        1. 숫자(`u32`) 타입에 원화(KRW) 형식의 문자열로 변환해 주는 메서드 `.to_krw()`를 추가하는 확장 트레이트 `PriceExt`를 정의하세요.
        2. `u32` 타입에 `PriceExt`를 구현하여, 숫자가 주어졌을 때 천 단위 콤마와 `"원"`이 붙은 문자열을 반환하도록 만드세요 (예: `15000` -> `"15,000원"`).
        3. `main` 함수에서 일반 정수(`let price = 50000;`)에 `.to_krw()` 메서드를 직접 호출하여 결과를 출력해 보세요.
- 과제 3: 연산자 오버로딩 (`Cart` 포인트 합산)
    - **목표**: 표준 라이브러리의 연산자 트레이트를 구현하여 커스텀 타입 간 연산 지원하기.
    - **요구사항**:
        1. 적립 포인트를 표현하는 구조체 `RewardPoints`를 정의하세요 (`points: u32`).
        2. 표준 라이브러리의 `std::ops::Add` 트레이트를 `RewardPoints`에 구현하여, 두 포인트 객체끼리 `+` 연산자를 사용해 합산할 수 있도록 만드세요.
        3. 디버그 출력을 위해 `#[derive(Debug, PartialEq)]`를 적용하세요.
        4. `main` 함수에서 두 개의 `RewardPoints` 인스턴스를 더한 결과가 정상적으로 계산되는지 확인하고 출력해 보세요.

### 과제 풀이

```rust
use std::ops::Add;

// --- 과제 1: 공통 인터페이스와 다형성 ---
trait Summary {
    fn summarize(&self) -> String;
}

#[derive(Debug)]
struct PhysicalProduct {
    name: String,
    price: u32,
    weight: f64,
}

#[derive(Debug)]
struct DigitalProduct {
    name: String,
    price: u32,
    file_size_mb: u32,
}

impl Summary for PhysicalProduct {
    fn summarize(&self) -> String {
        format!("[실물 상품] {} (가격: {}원, 무게: {}kg)", self.name, self.price, self.weight)
    }
}

impl Summary for DigitalProduct {
    fn summarize(&self) -> String {
        format!("[디지털 상품] {} (가격: {}원, 용량: {}MB)", self.name, self.price, self.file_size_mb)
    }
}

fn print_product_summary<T: Summary>(item: &T) {
    println!("{}", item.summarize());
}

// --- 과제 2: 확장 트레이트 ---
trait PriceExt {
    fn to_krw(&self) -> String;
}

impl PriceExt for u32 {
    fn to_krw(&self) -> String {
        let s = self.to_string();
        let mut result = String::new();
        let len = s.len();
        for (i, c) in s.chars().enumerate() {
            if i > 0 && (len - i) % 3 == 0 {
                result.push(',');
            }
            result.push(c);
        }
        result.push_str("원");
        result
    }
}

// --- 과제 3: 연산자 오버로딩 ---
#[derive(Debug, PartialEq, Clone, Copy)]
struct RewardPoints {
    points: u32,
}

impl Add for RewardPoints {
    type Output = RewardPoints;

    fn add(self, other: RewardPoints) -> RewardPoints {
        RewardPoints {
            points: self.points + other.points,
        }
    }
}

fn main() {
    // 과제 1 실행
    let physical = PhysicalProduct {
        name: String::from("Mechanical Keyboard"),
        price: 89_000,
        weight: 1.2,
    };
    let digital = DigitalProduct {
        name: String::from("Rust Programming E-Book"),
        price: 25_000,
        file_size_mb: 15,
    };

    print_product_summary(&physical);
    print_product_summary(&digital);

    // 과제 2 실행
    let price: u32 = 150000;
    println!("포맷된 가격: {}", price.to_krw());

    // 과제 3 실행
    let p1 = RewardPoints { points: 1500 };
    let p2 = RewardPoints { points: 3000 };
    let total_points = p1 + p2;
    println!("합산된 포인트: {:?}", total_points);
}
```

## Error Handling✔️

### 목표

- **회복 가능성과 불가능한 에러의 분리**: 프로그램이 중단되어야 하는 상황(`panic!`)과 복구하여 안전하게 처리해야 하는 상황(`Result<T, E>`)을 구분한다.
- **`Result`와 `Option` 활용**: 에러나 값이 존재하지 않을 수 있는 상황을 안전하게 다루기 위해 `match`, `unwrap`, `expect` 및 조합자 메서드를 사용한다.
- **`?` 연산자를 이용한 에러 전파**: 반복적인 `match` 문을 줄이고, 에러를 상위 호출자로 깔끔하게 위임한다.
- **커스텀 에러 정의**: 도메인에 맞는 명확한 에러 열거형(`enum Error`)을 정의하고 표준 에러 트레이트와 연동한다.

### 상황: 온라인 쇼핑몰 주문 및 결제 검증 시스템

주문(Order)을 생성하고 결제를 진행하는 과정에서 발생할 수 있는 다양한 예외 상황을 안전하게 처리하는 시스템을 구현합니다.

- **상황 1 (재고 검증 - `Option` 활용)**: 장바구니에 담은 상품의 재고가 존재하지 않거나 품절된 경우, 데이터가 없음을 나타내는 `Option`을 활용해 예외를 처리합니다.
- **상황 2 (쿠폰 및 가격 검증 - `Result` 활용)**: 만료된 쿠폰을 적용하거나 보유 잔액보다 결제 금액이 큰 경우, 구체적인 에러 사유를 담은 `Result`를 반환합니다.
- **상황 3 (에러 전파 - `?` 연산자 활용)**: 재고 확인 -> 쿠폰 검증 -> 결제 승인으로 이어진 복잡한 단계 중 어디에서든 에러가 발생하면 즉시 상위 함수로 에러를 전파하여 안전하게 거래를 중단시킵니다.

### 힌트

- 도메인 구조체
    
    ```rust
    struct Product {
        name: String,
        stock: u32,
        price: u32,
    }
    
    struct User {
        balance: u32,
    }
    ```
    
- ? 연산자를 이용한 에러 전파
    
    러스트에서 **`?` 연산자**는 `Result`나 `Option` 타입을 반환하는 함수에서 에러나 값이 없을 때의 처리를 매우 간결하게 만들어 주는 에러 전파(Error Propagation) 문법 설탕(Syntactic Sugar)이다.
    
    장황한 `match` 문을 작성하는 대신, 에러가 발생했을 때 즉시 상위 호출자로 에러를 반환하는 코드를 한 글자로 압축한다.
    
    - 동작 방식
        
        `?` 연산자가 붙은 표현식의 결과는 다음과 같이 동작한다.
        
        1. **성공인 경우 (`Ok(T)` 또는 `Some(T)`)**:
            - 에러나 None이 아니라면, 감싸져 있던 내부 값(`T`)을 꺼내어 변수에 대입하거나 그대로 진행한다.
        2. **실패인 경우 (`Err(E)` 또는 `None`)**:
            - 현재 함수의 실행을 즉시 중단하고, 해당 에러(`Err(E)`)를 **현재 함수의 반환값으로 곧바로 리턴한다**.
    - 코드 비교: 없이 쓰기 vs `?` 연산자 사용
        - 1. `?` 연산자가 없을 때 (`match` 문 사용)
        
        ```rust
        fn parse_number(s: &str) -> Result<i32, std::num::ParseIntError> {
            let parsed = match s.parse::<i32>() {
                Ok(n) => n,
                Err(e) => return Err(e), // 에러를 수동으로 반환
            };
            Ok(parsed * 2)
        }
        ```
        
        - 2. `?` 연산자를 사용할 때
        
        ```rust
        fn parse_number(s: &str) -> Result<i32, std::num::ParseIntError> {
            let parsed = s.parse::<i32>()?; // 에러가 나면 알아서 Err(e)를 리턴함
            Ok(parsed * 2)
        }
        ```
        
    - 주의할 점: 에러 타입의 호환성
        
        `?` 연산자는 단순히 `return Err(e);`를 대신해 주는 것이 아니라, 반환하려는 에러 타입으로 자동 변환(`From` 트레이트)을 시도합니다.
        
        따라서 `?`를 사용하는 함수의 반환 타입(`Result<T, E_func>`)이 내부에서 호출하는 함수의 에러 타입(`E_inner`)을 수용할 수 있거나, `From` 구현을 통해 변환이 가능한 상태여야 컴파일 에러가 나지 않는다.
        
- panic!
    
    러스트에서 `panic!`은 프로그램이 더 이상 정상적으로 실행될 수 없는 복구 불가능한 에러(Unrecoverable Error)가 발생했을 때 사용한다.
    
    `panic!`이 호출되면 프로그램은 즉시 에러 메시지와 스택 추적(Backtrace) 정보를 출력하고 안전하게(또는 즉시) 종료한다.  `Result`가 "에러를 처리(복구)할 수 있는 경우"에 사용된다면, `panic!`은 "절대 일어나서는 안 되는 버그나 치명적인 상태"를 마주했을 때 발생시킨다.
    
    - 예제 코드
    
    ```rust
    fn check_age(age: i32) {
        if age < 0 {
            // 프로그램이 유지될 수 없는 치명적인 상태이므로 패닉 발생
            panic!("나이는 음수가 될 수 없습니다! 입력된 값: {}", age);
        }
        println!("나이 확인 완료: {}세", age);
    }
    
    fn main() {
        check_age(25); // 정상 작동
    
        // 이 코드가 실행되면 패닉이 발생하며 프로그램이 즉시 중단된다.
        check_age(-5);
    }
    ```
    

### 결과

```rust
use std::fmt;

// 1. 도메인 맞춤 커스텀 에러 정의
#[derive(Debug)]
enum ShopError {
    OutOfStock(String),
    InvalidCoupon(String),
    InsufficientFunds { required: u32, available: u32 },
}

impl std::error::Error for ShopError {}

impl fmt::Display for ShopError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ShopError::OutOfStock(item) => write!(f, "품절된 상품입니다: {}", item),
            ShopError::InvalidCoupon(code) => write!(f, "유효하지 않은 쿠폰 코드입니다: {}", code),
            ShopError::InsufficientFunds { required, available } => {
                write!(f, "잔액이 부족합니다. (필요: {}원, 보유: {}원)", required, available)
            }
        }
    }
}

// 2. 도메인 구조체 정의
struct Product {
    name: String,
    stock: u32,
    price: u32,
}

struct User {
    balance: u32,
}

// 3. 개별 검증 함수 (Result 반환)
fn check_stock(product: &Product, quantity: u32) -> Result<(), ShopError> {
    if product.stock < quantity {
        return Err(ShopError::OutOfStock(product.name.clone()));
    }
    Ok(())
}

fn apply_coupon(coupon_code: Option<&str>, price: u32) -> Result<u32, ShopError> {
    match coupon_code {
        Some("SUMMER_10") => Ok(price * 9 / 10), // 10% 할인
        Some(invalid) => Err(ShopError::InvalidCoupon(invalid.to_string())),
        None => Ok(price),
    }
}

// 4. ? 연산자를 이용한 에러 전파 및 결제 파이프라인 함수
fn checkout(
    product: &Product,
    quantity: u32,
    coupon_code: Option<&str>,
    user: &mut User,
) -> Result<u32, ShopError> {
    // 재고 검증 (에러 발생 시 즉시 반환)
    check_stock(product, quantity)?;

    // 기본 가격 계산 및 쿠폰 적용 (에러 발생 시 즉시 반환)
    let base_price = product.price * quantity;
    let final_price = apply_coupon(coupon_code, base_price)?;

    // 잔액 검증
    if user.balance < final_price {
        return Err(ShopError::InsufficientFunds {
            required: final_price,
            available: user.balance,
        });
    }

    // 결제 승인 (잔액 차감)
    user.balance -= final_price;
    Ok(final_price)
}

fn main() {
    let laptop = Product {
        name: String::from("MacBook Pro"),
        stock: 3,
        price: 1_500_000,
    };

    let mut user = User {
        balance: 2_000_000,
    };

    println!("--- 시나리오 1: 정상 결제 (쿠폰 적용) ---");
    match checkout(&laptop, 1, Some("SUMMER_10"), &mut user) {
        Ok(price) => println!("결제 완료! 최종 금액: {}원 (남은 잔액: {}원)", price, user.balance),
        Err(e) => println!("결제 실패: {}", e),
    }

    println!("\n--- 시나리오 2: 재고 초과 주문 ---");
    match checkout(&laptop, 5, None, &mut user) {
        Ok(price) => println!("결제 완료! 최종 금액: {}원", price),
        Err(e) => println!("결제 실패: {}", e),
    }

    println!("\n--- 시나리오 3: 잘못된 쿠폰 입력 ---");
    match checkout(&laptop, 1, Some("INVALID_CODE"), &mut user) {
        Ok(price) => println!("결제 완료! 최종 금액: {}원", price),
        Err(e) => println!("결제 실패: {}", e),
    }
}
```

### 결과(thiserror 라이브러리 활용)

```rust
use thiserror::Error;

// thiserror를 사용한 간결하고 직관적인 커스텀 에러 정의
#[derive(Error, Debug)]
pub enum ShopError {
    #[error("품절된 상품입니다: {0}")]
    OutOfStock(String),

    #[error("유효하지 않은 쿠폰 코드입니다: {0}")]
    InvalidCoupon(String),

    #[error("잔액이 부족합니다. (필요: {required}원, 보유: {available}원)")]
    InsufficientFunds { required: u32, available: u32 },

    // 외부 에러(std::io::Error)를 자동으로 변환(From 구현)해 주는 어트리뷰트
    #[error("시스템 내부 I/O 오류가 발생했습니다.")]
    IoError(#[from] std::io::Error),
}

struct Product {
    name: String,
    stock: u32,
    price: u32,
}

struct User {
    balance: u32,
}

fn check_stock(product: &Product, quantity: u32) -> Result<(), ShopError> {
    if product.stock < quantity {
        return Err(ShopError::OutOfStock(product.name.clone()));
    }
    Ok(())
}

fn apply_coupon(coupon_code: Option<&str>, price: u32) -> Result<u32, ShopError> {
    match coupon_code {
        Some("SUMMER_10") => Ok(price * 9 / 10), // 10% 할인
        Some(invalid) => Err(ShopError::InvalidCoupon(invalid.to_string())),
        None => Ok(price),
    }
}

fn checkout(
    product: &Product,
    quantity: u32,
    coupon_code: Option<&str>,
    user: &mut User,
) -> Result<u32, ShopError> {
    check_stock(product, quantity)?;

    let base_price = product.price * quantity;
    let final_price = apply_coupon(coupon_code, base_price)?;

    if user.balance < final_price {
        return Err(ShopError::InsufficientFunds {
            required: final_price,
            available: user.balance,
        });
    }

    user.balance -= final_price;
    Ok(final_price)
}

fn main() {
    let laptop = Product {
        name: String::from("MacBook Pro"),
        stock: 3,
        price: 1_500_000,
    };

    let mut user = User {
        balance: 2_000_000,
    };

    println!("--- 정상 결제 테스트 ---");
    match checkout(&laptop, 1, Some("SUMMER_10"), &mut user) {
        Ok(price) => println!("결제 성공! 최종 금액: {}원 (남은 잔액: {}원)", price, user.balance),
        Err(e) => println!("결제 실패: {}", e),
    }

    println!("\n--- 잘못된 쿠폰 테스트 ---");
    match checkout(&laptop, 1, Some("BAD_COUPON"), &mut user) {
        Ok(price) => println!("결제 성공! 최종 금액: {}원", price),
        Err(e) => println!("결제 실패: {}", e),
    }
}
```

## Serde: Serialization/Deserialization✔️

### 목표

- **직렬화(Serialization) 이해**: 러스트 구조체나 데이터를 JSON 등 외부 형식(바이트 또는 문자열)으로 변환하는 방법
- **역직렬화(Deserialization) 이해**: 외부에서 들어온 JSON 데이터(문자열)를 러스트의 안전한 타입 구조체로 안전하게 변환하는 방법
- **`serde`와 `serde_json` 활용**: `#[derive(Serialize, Deserialize)]` 매크로를 사용하여 보일러플레이트 없이 데이터를 변환한다.
- **고급 속성 제어 (`#[serde(...)]`)**: 필드 이름 변경(예: `camelCase`와 `snake_case` 매핑), 옵션 값 처리, 기본값 설정 등 실무에서 필요한 데이터 구조 조작법을 다룬다.

### 상황: 이커머스 주문 접수 및 응답 API 데이터 매핑

클라이언트가 서버로 보낸 JSON 형태의 주문 요청 데이터를 러스트 구조체로 역직렬화하고, 주문 처리 결과 구조체를 다시 JSON 문자열로 직렬화하여 응답하는 파이프라인을 구현합니다.

- **상황 1 (주문 요청 역직렬화)**: 클라이언트가 전송한 JSON 문자열(`{"product_id": "P123", "quantity": 2, "coupon_code": "SUMMER_10"}`)을 받아 러스트의 `OrderRequest` 구조체로 변환합니다.
- **상황 2 (데이터 변환 및 매핑)**: `#[serde(rename_all = "camelCase")]` 등의 어트리뷰트를 활용해 JSON의 키 이름 스타일과 러스트 구조체의 필드 스타일을 자연스럽게 매핑합니다.
- **상황 3 (주문 결과 직렬화)**: 주문 성공 여부와 결제 금액이 담긴 `OrderResponse` 구조체를 생성한 뒤, 이를 다시 클라이언트에게 보낼 JSON 문자열로 직렬화합니다.

### 힌트

- dependency
    
    ```toml
    [dependencies]
    serde = { version = "1.0", features = ["derive"] }
    serde_json = "1.0"
    ```
    
- struct
    
    ```rust
    struct OrderRequest {
        product_id: String,
        quantity: u32,
        #[serde(default)]
        coupon_code: Option<String>,
    }
    
    struct OrderResponse {
        success: bool,
        order_id: String,
        total_price: u32,
        message: String,
    }
    ```
    
- drive와 매크로
    
    코드를 자동으로 생성하거나 확장해 주는 메타프로그래밍(Metaprogramming) 도구
    
    - 매크로
        
        매크로는 컴파일 타임에 코드를 생성하는 전체 기술이다.
        
        - 선언적 매크로 (`macro_rules!`)
            - 흔히 보는 `println!`이나 `vec!`처럼 패턴 매칭을 기반으로 코드를 반복 생성하는 매크로다. 정해진 규칙에 따라 코드를 치환하는 방식으로 비교적 간단하게 작성
        - 프로시저럴 매크로 (`Procedural Macro`)
            - Rust 코드를 입력(TokenStream)받아 AST(추상 구문 트리)로 분석한 뒤, 완전히 새로운 코드를 프로그래밍 방식으로 작성한다. 별도의 라이브러리 크레이트(`proc-macro = true`)로 분리해서 만들어야 합니다.
    - derive
        
        프로시저럴 매크로의 한 종류이다. 구조체나 열거형 위에 `#[derive(TraitName)]` 형태로 선언하면, 컴파일러가 해당 구조체의 이름과 필드 정보(AST)를 읽어 그에 맞는 트레이트 구현 코드를 자동으로 만들어준다.
        
        프로시저럴 매크로는 크게 세 가지 형태로 나뉘는데, `derive`는 그중 가장 많이 쓰이는 형태이다.
        
        1. 커스텀 Derive 매크로 (`#[proc_macro_derive]`)
            - `#[derive(Debug, Serialize, MyOrmModel)]`처럼 사용하며, 특정 트레이트를 자동으로 구현해 준다.
        2. 어트리뷰트 매크로 (`#[proc_macro_attribute]`)
            - 함수나 구조체 자체에 커스텀 어트리뷰트(예: 웹 프레임워크의 라우팅 설정 `#[get("/users")]`)를 붙여 코드를 통째로 변환하거나 감싼다
        3. 함수형 매크로 (`custom_macro!()`)
            - 함수 호출 형태처럼 생겨서 안의 인자를 받아 코드를 생성한다.

### 결과

- 상황 1: 역직렬화
    
    ```rust
    use serde::Deserialize;
    
    // 클라이언트가 보낸 주문 요청 JSON을 매핑할 구조체
    #[derive(Deserialize, Debug)]
    struct OrderRequest {
        product_id: String,
        quantity: u32,
        coupon_code: Option<String>, // 쿠폰은 없을 수도 있으므로 Option 처리
    }
    
    fn main() {
        // 상황 1: 클라이언트가 보낸 JSON 문자열 (가상 시나리오)
        let json_data = r#"{
            "product_id": "P12345",
            "quantity": 2,
            "coupon_code": "SUMMER_10"
        }"#;
    
        // serde_json을 이용해 JSON 문자열을 러스트 구조체로 역직렬화 (Deserialization)
        let order_result: Result<OrderRequest, serde_json::Error> = serde_json::from_str(json_data);
    
        match order_result {
            Ok(order) => {
                println!("--- [상황 1] 주문 요청 역직렬화 성공! ---");
                println!("구조체 데이터: {:?}", order);
                println!("상품 ID: {}", order.product_id);
                println!("주문 수량: {}", order.quantity);
                println!("적용 쿠폰: {:?}", order.coupon_code);
            }
            Err(e) => {
                println!("역직렬화 실패: {}", e);
            }
        }
    }
    ```
    
- 상황 2: 데이터 매핑
    
    ```rust
    use serde::Deserialize;
    
    // #[serde(rename_all = "camelCase")]를 사용하여
    // JSON의 camelCase 키들을 러스트의 snake_case 필드에 자동으로 매핑합니다.
    #[derive(Deserialize, Debug)]
    #[serde(rename_all = "camelCase")]
    struct OrderRequest {
        product_id: String,
        quantity: u32,
        
        // 특정 필드만 이름을 다르게 매핑하고 싶다면 #[serde(rename = "...")]을 쓸 수도 있습니다.
        #[serde(default)] // JSON에 값이 아예 없을 경우 기본값(Default::default() -> None)으로 처리
        coupon_code: Option<String>,
    }
    
    fn main() {
        // 상황 2: 카멜케이스(camelCase)로 전달된 JSON 데이터
        let json_data = r#"{
            "productId": "P99999",
            "quantity": 5
        }"#;
    
        let order_result: Result<OrderRequest, serde_json::Error> = serde_json::from_str(json_data);
    
        match order_result {
            Ok(order) => {
                println!("--- [상황 2] 카멜케이스 매핑 및 기본값 처리 성공! ---");
                println!("구조체 데이터: {:?}", order);
                println!("상품 ID: {}", order.product_id);
                println!("주문 수량: {}", order.quantity);
                println!("적용 쿠폰: {:?} (쿠폰이 없어 기본값 None 처리됨)", order.coupon_code);
            }
            Err(e) => {
                println!("역직렬화 실패: {}", e);
            }
        }
    }
    ```
    
- 상황 3: 직렬화
    
    ```rust
    use serde::{Deserialize, Serialize};
    
    // 1. 요청 데이터를 받는 구조체 (역직렬화)
    #[derive(Deserialize, Debug)]
    #[serde(rename_all = "camelCase")]
    struct OrderRequest {
        product_id: String,
        quantity: u32,
        #[serde(default)]
        coupon_code: Option<String>,
    }
    
    // 2. 응답 데이터를 보내는 구조체 (직렬화)
    // 클라이언트 친화적으로 응답도 camelCase로 변환되도록 설정합니다.
    #[derive(Serialize, Debug)]
    #[serde(rename_all = "camelCase")]
    struct OrderResponse {
        success: bool,
        order_id: String,
        total_price: u32,
        message: String,
    }
    
    fn main() {
        // [상황 1 & 2] 클라이언트가 보낸 JSON 요청 (역직렬화 대상)
        let json_request = r#"{
            "productId": "P12345",
            "quantity": 2,
            "coupon_code": "SUMMER_10"
        }"#;
    
        // JSON -> 러스트 구조체로 변환
        let order_req: OrderRequest = serde_json::from_str(json_request).unwrap();
        println!("--- [역직렬화 완료] 수신된 주문 요청 ---");
        println!("{:?}", order_req);
    
        // 비즈니스 로직 처리 (가상의 결제 및 총액 계산)
        let unit_price = 50_000;
        let base_price = unit_price * order_req.quantity;
        let discount = if order_req.coupon_code.as_deref() == Some("SUMMER_10") {
            base_price / 10 // 10% 할인
        } else {
            0
        };
        let total_price = base_price - discount;
    
        // [상황 3] 응답 데이터 구조체 생성
        let response_data = OrderResponse {
            success: true,
            order_id: String::from("ORD-2026-001"),
            total_price,
            message: String::from("주문이 성공적으로 완료되었습니다."),
        };
    
        // serde_json을 이용해 러스트 구조체를 JSON 문자열로 직렬화 (Serialization)
        // pretty_print(to_string_pretty)를 사용하여 들여쓰기가 예쁜 JSON 문자열로 변환합니다.
        let json_response = serde_json::to_string_pretty(&response_data).unwrap();
    
        println!("\n--- [상황 3] 주문 처리 결과 직렬화 성공 (클라이언트 응답 JSON) ---");
        println!("{}", json_response);
    }
    ```
    

## File I/O & Working with Paths✔️

### 목표

- **표준 파일 I/O 기본기 숙달**: 표준 라이브러리(`std::fs`, `std::io`)를 활용하여 파일을 생성, 읽기, 쓰기, 수정하는 방법
- **에러 핸들링**: 파일이 존재하지 않거나 권한이 없는 등 파일 I/O 과정에서 발생하는 예외 상황을 `Result`와 `?` 연산자로 안전하게 처리합니다.
- **버퍼드 I/O(Buffered I/O) 활용**: 성능 최적화를 위해 버퍼를 사용하는 `BufReader`와 `BufWriter`의 개념과 사용법을 이해

### 상황: 이커머스 시스템 설정 로드 및 주문 감사 로그(Audit Log) 기록

- **상황 1 (설정 파일 읽기)**: 서버가 구동될 때 로컬 경로에 있는 설정 파일(config/settings.json)을 읽어와 러스트 구조체로 역직렬화하여 시스템 환경 변수를 세팅한다.
- **상황 2 (주문 감사 로그 기록)**: 사용자가 결제를 완료할 때마다 상세 주문 내역을 텍스트 로그 파일(`order_audit.log`)에 누락 없이 이어쓰기(Append) 방식으로 기록한다.

### 힌트

- dependency
    
    ```toml
    [dependencies]
    serde = { version = "1.0", features = ["derive"] }
    serde_json = "1.0"
    ```
    
- struct
    
    ```rust
    // [상황 1] 설정 파일 구조체
    #[derive(Deserialize, Serialize, Debug)]
    #[serde(rename_all = "camelCase")]
    struct AppConfig {
        server_host: String,
        server_port: u16,
        max_connections: usize,
    }
    
    // [상황 2] 감사 로그 구조체
    #[derive(Serialize, Debug)]
    struct AuditLog {
        timestamp: String,
        order_id: String,
        action: String,
        amount: u32,
    }
    ```
    
- `?` 에러 전파 연산자
    
    러스트에서 `?` 연산자는 '에러 전파 연산자(Error Propagation Operator)'이다.
    
    에러가 발생할 수 있는 연산 뒤에 붙여서, 코드를 매우 간결하게 작성할 수 있도록 한다.
    
    1. **성공(`Ok`)한 경우**: `Ok(값)` 안에 들어 있는 실제 데이터를 안전하게 꺼내어(`unwrap`) 변수에 담고 다음 코드를 계속 실행
    2. **에러(`Err`)인 경우**: 남은 코드를 더 이상 실행하지 않고 즉시 현재 함수를 빠져나와, 해당 에러를 호출자(부모 함수)에게 곧바로 반환(Early Return)
    
    이 `?` 연산자를 사용하려면 현재 함수가 결과를 반환하는 `Result` 타입이어야 한다
    

### 결과

```rust
use serde::{Deserialize, Serialize};
use std::fs::{self, OpenOptions};
use std::io::{self, BufReader, BufWriter, Write};
use std::path::Path;

// [상황 1] 설정 파일 구조체
#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
struct AppConfig {
    server_host: String,
    server_port: u16,
    max_connections: usize,
}

// [상황 2] 감사 로그 구조체
#[derive(Serialize, Debug)]
struct AuditLog {
    timestamp: String,
    order_id: String,
    action: String,
    amount: u32,
}

fn main() -> io::Result<()> {
    let config_path = "config.json";
    let log_path = "order_audit.log";

    // -------------------------------------------------------------
    // [상황 1] 설정 파일 읽기 (없을 경우 기본 파일을 먼저 생성해 둠)
    // -------------------------------------------------------------
    if !Path::new(config_path).exists() {
        let default_config = AppConfig {
            server_host: String::from("127.0.0.1"),
            server_port: 8080,
            max_connections: 1000,
        };
        let json_str = serde_json::to_string_pretty(&default_config)?;
        fs::write(config_path, json_str)?;
        println!("초기 설정 파일('{}')이 생성되었습니다.", config_path);
    }

    // 파일 열기 및 BufReader를 이용한 고성능 읽기 & 역직렬화
    let config_file = fs::File::open(config_path)?;
    let reader = BufReader::new(config_file);
    let config: AppConfig = serde_json::from_reader(reader)?;

    println!("--- [상황 1] 설정 파일 로드 성공 ---");
    println!("서버 주소: {}:{}", config.server_host, config.server_port);
    println!("최대 동시 접속 수: {}\n", config.max_connections);

    // -------------------------------------------------------------
    // [상황 2] 주문 감사 로그(Audit Log) 이어쓰기 (Append)
    // -------------------------------------------------------------
    // OpenOptions를 사용하여 파일이 없으면 생성하고, 있으면 끝에 이어서 작성(append(true))합니다.
    let log_file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(log_path)?;

    let mut writer = BufWriter::new(log_file);

    // 가상의 주문 완료 로그 데이터 생성
    let new_log = AuditLog {
        timestamp: String::from("2026-03-31T12:34:56Z"),
        order_id: String::from("ORD-2026-001"),
        action: String::from("CHECKOUT_SUCCESS"),
        amount: 1_350_000,
    };

    // 구조체를 JSON 한 줄(Line-delimited JSON)로 변환하여 로그 파일에 기록
    let log_json = serde_json::to_string(&new_log)?;
    writeln!(writer, "{}", log_json)?;
    writer.flush()?; // 버퍼에 남은 데이터를 파일에 물리적으로 반영

    println!("--- [상황 2] 주문 감사 로그 기록 성공 ---");
    println!("기록된 로그 파일 경로: {}", log_path);
    println!("로그 내용: {}", log_json);

    Ok(())
}
```