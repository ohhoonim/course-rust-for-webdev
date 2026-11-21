fn main() {
    // if_condition();
    // println!("{}", if_condition_expression());
    // interation_loop();
    // interation_while();
    // iteraion_for();
    pattern_matching();
}

fn if_condition() {
    let x = 5;
    if x == 5 {
        println!("{}", "숫자 5 입니다.");
    }
}

fn if_condition_expression() -> String {
    let x = 5;
    let x = if x == 5 {
        String::from("다섯")
    } else {
        x.to_string()
    };
    x

    // shadowing을 보여주기 위해 이렇게 코딩했으나 그냥
    // 함수 마지막 줄에 아래와 같이 적어줘도 된다(맨 끝에 세미콜론이 없다.)
    // if x == 5 { String::from("다섯") } else { x.to_string()}
}

fn interation_loop() {
    let mut counter = 0;
    let final_count = loop {
        counter += 1;
        if counter == 10 {
            break counter * 20;
        }
    };
    println!("final_ocunt: {}", final_count);
}

fn interation_while() {
    let mut count = 3;
    while count > 0 {
        println!("{}", count);
        count -= 1;
    }
}

fn iteraion_for() {
    for n in 1..3 {
        // 이상 - 미만
        println!("{}", n);
    }
}

fn pattern_matching() {
    let grade = 'Z';
    match grade {
        'A' => println!("우수"),
        'B' => println!("미양"),
        'C' => println!("가"),
        _ => println!("매치되는 것이 없음")
    }
}
