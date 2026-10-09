fn main() {
    let numbers = vec![10, 20, 54, 12, 4];
    println!("{}", max_value_no_use_generic(&numbers));

    let chars = vec!['a', 'c', 'b', 'a'];
    println!("{}", max_value_use_generic(&chars));

    let article = Article { author: "matthew".to_string(), headline: "Today news".to_string()};
    let tweet = Tweet { text: "blabla".to_string()};

    println!("{}", article.summarize());
    println!("{}", tweet.summarize());
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
fn max_value_use_generic<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    } 
    largest
}

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