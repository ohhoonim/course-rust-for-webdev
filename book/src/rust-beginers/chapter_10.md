
# 열거형(Enums)

## 기본 사용법

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

## 데이터가 포함된 Enum

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

## Enum에 대한 메서드 구현과 패턴매칭

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
