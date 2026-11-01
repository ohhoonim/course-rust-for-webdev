fn main() {
    // print_s1_length();

    let s1 = String::from("my car is greate");
    let s1_length = use_str_length_calculator(&s1);
    println!("{} {}", s1, s1_length);
}

fn print_s1_length() {
    let s1 = String::from("rust");
    let len = s1.len();

    println!("'{}' length is {} ", s1, len);
}

fn use_str_length_calculator(str: &String) -> usize {
    str.len()
}

