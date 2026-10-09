fn main() {
    match immutable_variables2() {
        Ok(val) => println!("성공: {}", val),
        Err(e) => println!("에러: {}", e),
    }
    println!("mutable_variables: {}", mutable_variables());
    println!("shdowing: {}", shdowing());
    println!("PI: {}", PI);
}

fn immutable_variables2() -> Result<i32, &'static str> {
    let x = 5;
    // x = 6 // error
    Err("가변인자는 mut를 사용")
}

fn mutable_variables() -> i32 {
    let mut x = 5;
    x = 6;
    x
}

fn shdowing() -> i32 {
    let x = 5;
    let x = 51;
    x
}

const PI: f64 = 3.14159;