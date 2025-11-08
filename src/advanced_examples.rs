// ============================================
// Rust 심화 학습 예제 모음
// ============================================

// 예제 1: 복잡한 구조체와 메서드
struct Library {
    name: String,
    books: Vec<Book>,
}

struct Book {
    title: String,
    author: String,
    pages: u32,
    published: u16,
}

impl Library {
    fn new(name: String) -> Self {
        Library {
            name,
            books: Vec::new(),
        }
    }

    fn add_book(&mut self, book: Book) {
        self.books.push(book);
    }

    fn total_pages(&self) -> u32 {
        self.books.iter().map(|b| b.pages).sum()
    }

    fn find_by_author(&self, author: &str) -> Vec<&Book> {
        self.books
            .iter()
            .filter(|b| b.author == author)
            .collect()
    }
}

// 예제 2: 열거형 패턴 매칭
#[derive(Debug)]
enum Command {
    Quit,
    Move { x: i32, y: i32 },
    Say(String),
    ChangeColor(u8, u8, u8),
}

fn execute_command(cmd: Command) {
    match cmd {
        Command::Quit => println!("Quitting..."),
        Command::Move { x, y } => println!("Moving to ({}, {})", x, y),
        Command::Say(msg) => println!("Saying: {}", msg),
        Command::ChangeColor(r, g, b) => println!("Changing color to RGB({}, {}, {})", r, g, b),
    }
}

// 예제 3: Result와 에러 처리
fn parse_number_safe(s: &str) -> Result<i32, String> {
    match s.parse() {
        Ok(num) => Ok(num),
        Err(_) => Err(format!("'{}' is not a valid number", s)),
    }
}

// ? 연산자 사용
fn add_two_numbers(a: &str, b: &str) -> Result<i32, String> {
    let num_a = a.parse::<i32>().map_err(|_| format!("'{}' is not a number", a))?;
    let num_b = b.parse::<i32>().map_err(|_| format!("'{}' is not a number", b))?;
    Ok(num_a + num_b)
}

// 예제 4: 제네릭과 트레이트
trait Drawable {
    fn draw(&self);
}

struct Circle {
    radius: f64,
}

struct Rectangle {
    width: f64,
    height: f64,
}

impl Drawable for Circle {
    fn draw(&self) {
        println!("Drawing a circle with radius {}", self.radius);
    }
}

impl Drawable for Rectangle {
    fn draw(&self) {
        println!("Drawing a rectangle {}x{}", self.width, self.height);
    }
}

fn draw_all(items: Vec<Box<dyn Drawable>>) {
    for item in items {
        item.draw();
    }
}

// 예제 5: 제네릭 함수
fn print_pair<T: std::fmt::Display>(a: T, b: T) {
    println!("First: {}, Second: {}", a, b);
}

fn get_biggest<T: PartialOrd + Copy>(a: T, b: T) -> T {
    if a > b { a } else { b }
}

// 예제 6: Iterator와 함수형 프로그래밍
fn iterator_examples() {
    let v = vec![1, 2, 3, 4, 5];

    // iter - 불변 참조
    let sum: i32 = v.iter().sum();
    println!("Sum: {}", sum);

    // map
    let doubled: Vec<i32> = v.iter().map(|x| x * 2).collect();
    println!("Doubled: {:?}", doubled);

    // filter
    let evens: Vec<i32> = v.iter().filter(|x| *x % 2 == 0).map(|x| *x).collect();
    println!("Evens: {:?}", evens);

    // fold
    let product = v.iter().fold(1, |acc, x| acc * x);
    println!("Product: {}", product);

    // chain
    let v1 = vec![1, 2, 3];
    let v2 = vec![4, 5, 6];
    let combined: Vec<i32> = v1.iter().chain(v2.iter()).map(|x| *x).collect();
    println!("Combined: {:?}", combined);
}

// 예제 7: String vs &str 이해하기
fn string_ownership_example() {
    // String: 힙에 할당, 소유권 있음
    let s1 = String::from("hello");
    let _s2 = s1;  // 소유권 이동
    // println!("{}", s1);  // ❌ 에러

    // &str: 참조, 소유권 없음
    let s3 = "hello";
    let s4 = s3;
    println!("{}, {}", s3, s4);  // ✅ 둘 다 가능

    // 함수에서
    fn takes_string(s: String) {
        println!("{}", s);
    }

    fn takes_str(s: &str) {
        println!("{}", s);
    }

    let s = String::from("hello");
    takes_str(&s);
    println!("Still have s: {}", s);  // ✅ s 여전히 유효

    // takes_string(s);
    // println!("{}", s);  // ❌ s는 이제 소유권을 잃음
}

// 예제 8: 클로저 활용
fn closure_examples() {
    let x = 4;

    let add_x = |y| x + y;  // x를 캡처
    let result = add_x(5);
    println!("Closure result: {}", result);

    // FnOnce, FnMut, Fn 트레이트
    let mut counter = 0;

    let mut increment = || {
        counter += 1;
        counter
    };

    println!("First call: {}", increment());
    println!("Second call: {}", increment());
}

// 예제 9: 정렬과 비교
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Person {
    name: String,
    age: u32,
}

fn sorting_example() {
    let mut people = vec![
        Person {
            name: "Alice".to_string(),
            age: 30,
        },
        Person {
            name: "Bob".to_string(),
            age: 25,
        },
        Person {
            name: "Charlie".to_string(),
            age: 35,
        },
    ];

    // 정렬
    people.sort_by(|a, b| a.age.cmp(&b.age));
    println!("Sorted by age: {:?}", people);

    // 역순 정렬
    people.reverse();
    println!("Reversed: {:?}", people);
}

// 예제 10: Option 활용
fn option_examples() {
    let x: Option<i32> = Some(5);
    let _y: Option<i32> = None;

    // match
    match x {
        Some(n) => println!("x is {}", n),
        None => println!("x is None"),
    }

    // if let
    if let Some(n) = x {
        println!("x is {}", n);
    }

    // unwrap_or
    let x_or_zero = x.unwrap_or(0);
    println!("x_or_zero: {}", x_or_zero);

    // map
    let doubled = x.map(|n| n * 2);
    println!("doubled: {:?}", doubled);

    // and_then
    let result = x.and_then(|n| {
        if n > 3 {
            Some(n * 2)
        } else {
            None
        }
    });
    println!("result: {:?}", result);
}

// 예제 11: 수명이 있는 참조
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// 예제 12: 테스트 작성
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_number() {
        assert_eq!(parse_number_safe("42"), Ok(42));
        assert!(parse_number_safe("not a number").is_err());
    }

    #[test]
    fn test_add_two_numbers() {
        assert_eq!(add_two_numbers("10", "20"), Ok(30));
        assert!(add_two_numbers("abc", "20").is_err());
    }

    #[test]
    fn test_longest() {
        assert_eq!(longest("long", "short"), "long");
        assert_eq!(longest("short", "longer"), "longer");
    }
}

fn main() {
    println!("=== Rust 심화 예제 ===\n");

    // 예제 1: 라이브러리
    println!("예제 1: Library");
    let mut library = Library::new("City Library".to_string());
    library.add_book(Book {
        title: "The Rust Book".to_string(),
        author: "Steve Klabnik".to_string(),
        pages: 560,
        published: 2018,
    });
    library.add_book(Book {
        title: "Programming Rust".to_string(),
        author: "Jim Blandy".to_string(),
        pages: 622,
        published: 2017,
    });
    println!("Library: {}", library.name);
    println!("Total pages: {}", library.total_pages());
    println!();

    // 예제 2: 명령 실행
    println!("예제 2: Command Execution");
    execute_command(Command::Move { x: 10, y: 20 });
    execute_command(Command::Say("Hello!".to_string()));
    execute_command(Command::ChangeColor(255, 0, 0));
    println!();

    // 예제 3: 숫자 파싱
    println!("예제 3: Parse Numbers");
    println!("Parse '42': {:?}", parse_number_safe("42"));
    println!("Parse 'abc': {:?}", parse_number_safe("abc"));
    println!("Add '10' + '20': {:?}", add_two_numbers("10", "20"));
    println!();

    // 예제 4: 트레이트
    println!("예제 4: Traits");
    let items: Vec<Box<dyn Drawable>> = vec![
        Box::new(Circle { radius: 5.0 }),
        Box::new(Rectangle {
            width: 10.0,
            height: 20.0,
        }),
    ];
    draw_all(items);
    println!();

    // 예제 5: 제네릭
    println!("예제 5: Generics");
    print_pair(5, 10);
    print_pair("hello", "world");
    let bigger = get_biggest(42, 24);
    println!("Bigger of 42 and 24: {}", bigger);
    println!();

    // 예제 6: Iterator
    println!("예제 6: Iterators");
    iterator_examples();
    println!();

    // 예제 7: String
    println!("예제 7: String vs &str");
    string_ownership_example();
    println!();

    // 예제 8: 클로저
    println!("예제 8: Closures");
    closure_examples();
    println!();

    // 예제 9: 정렬
    println!("예제 9: Sorting");
    sorting_example();
    println!();

    // 예제 10: Option
    println!("예제 10: Option");
    option_examples();
    println!();

    // 예제 11: 수명
    println!("예제 11: Lifetimes");
    let result = longest("long string", "short");
    println!("Longest: {}", result);
}
