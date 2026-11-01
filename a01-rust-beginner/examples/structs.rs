fn main() {

    let matthew = User {
        username : "matthew".to_string(),
        age: 22,
        active: false,
    };

    println!("{:#?}", matthew);
    println!("{:?}", matthew);
    println!("{}", matthew.username);
    println!("{}", matthew.age);
    println!("{}", matthew.active);
    println!("{}", matthew.describe());

    let someone = User::new();
    println!("{}", someone.active)

}

#[derive(Debug)]
struct User {
    username: String,
    age: u32,
    active: bool,
}

impl User {
    fn new() -> Self {
        User {
            username: String::from(""),
            age: 0,
            active: false,
        }
    }
    fn describe(&self) -> String {
        format!("{} (active: {})", self.username, self.active)
    }
}