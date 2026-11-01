fn main() {
    let mut s = String::from("Hello");
    add_exclamtion(&mut s);

    println!("{}", s); // Hello!

}

fn add_exclamtion(text: &mut String ) {
    text.push_str("!");
}