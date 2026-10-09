fn main() {

    let player_direction = Direction::Right;
    println!("{:?}", player_direction);

    let msg1 = Message::Write(String::from("bye!"));
    let msg2 = Message::Move{ x: 10, y: 20};

    println!("{:?}", msg1);
    println!("{:?}", msg2);

    let user_status = Status::Active(33);
    user_status.check();
}

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right, 
}

#[derive(Debug)]
enum Message {
    Quit,
    Move {x: i32, y: i32},
    Write(String),
    ChangeColor(i32, i32, i32),
}

enum Status {
	Active(i32),
	Inactive,
}

impl Status{
	fn check(&self) {
      match self {
          Status::Active(id) => println!("활성화 상태 (ID: {})", id),
          Status::Inactive => println!("비활성화 상태"),
      }
  }
}