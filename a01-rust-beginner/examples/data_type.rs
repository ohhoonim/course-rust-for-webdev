
fn main() {
    scalar_types();
    tuple_type();
    destructure_tuple();
    array_init();
    array_vec_macro();
}

fn scalar_types() {
    let age: u32 = 10;
    let num: i32 = -100;
    let temperature: f64 = 36.6;
    let is_active: bool = true;
    let grade: char = 'A';

    println!(
        "scalars ::: age: {}, temp: {}, active: {}, grade: {}, num: {}",
        age, temperature, is_active, grade, num
    );
}

fn tuple_type() {
    let person: (&str, u32, u32) = ("Maya", 29, 50000);
    println!(
        "tuple ::: {} is {} years old, has the salary {}",
        person.0, person.1, person.2
    );
}

fn destructure_tuple() {
    let person: (&str, u32, u32) = ("Maya", 29, 50000);
    let (name, age, salary) = person;
    println!(
        "destructure ::: {} is {} years old, has the salary {}",
        name, age, salary
    );
}

fn array_init() {
    let scores: [i32; 3] = [90, 85, 100];
    println!("first score: {}", scores[0]);
}

fn array_vec_macro() {
    let mut numbers = vec![1, 2, 3, 4, 5]; // 타입추론
    println!("초기 벡터: {:?}", numbers); // 디버그 출력을 위해 {:?} 사용

    let zeros = vec![0; 3]; // 특정 값을 특정 길이만큼 반복해서 초기화
    println!("0으로 채워진 벡터: {:?}", zeros);
}
