fn main() {
    let user = find_uesr(12);
    let username = match user {
        Some(u) => u,
        None => "찾을 수 없습니다.".to_string(),
    };
    println!("{}", username);

    let result = divide(20.0, 0.0);
    match result {
        Ok(r) => println!("{}", r),
        Err(m) => println!("{}", m),
    }
}

fn find_uesr(id: i32) -> Option<String> {
    if id == 11 {
        Some("matthew".to_string())
    } else {
        None
    }
}

fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        return Err("0으로 나눌 수 없다".to_string())
    }
    Ok(a / b)
}
