// ============================================
// 12. 제네릭 & 트레이트 심화 학습
// 재사용 가능한 코드 작성 방법
// ============================================

pub fn run_generics_traits_examples() {
    println!("\n=== 제네릭 & 트레이트 심화 학습 ===\n");

    // ============================================
    // 1. 제네릭 함수
    // ============================================
    println!("1. 제네릭 함수");
    generic_functions();
    println!();

    // ============================================
    // 2. 제네릭 구조체
    // ============================================
    println!("2. 제네릭 구조체");
    generic_structs();
    println!();

    // ============================================
    // 3. 트레이트 정의 및 구현
    // ============================================
    println!("3. 트레이트 정의 및 구현");
    trait_definitions();
    println!();

    // ============================================
    // 4. 트레이트 바운드
    // ============================================
    println!("4. 트레이트 바운드");
    trait_bounds();
    println!();

    // ============================================
    // 5. 트레이트 객체
    // ============================================
    println!("5. 트레이트 객체");
    trait_objects();
    println!();

    // ============================================
    // 6. 실전 예제: AST Processor
    // ============================================
    println!("6. 실전 예제: AST Processor");
    ast_processor_example();
    println!();
}

// ============================================
// 제네릭 함수
// ============================================
fn generic_functions() {
    // 기본 제네릭 함수
    fn print_value<T: std::fmt::Display>(value: T) {
        println!("  값: {}", value);
    }

    print_value(42);
    print_value("hello");
    print_value(3.14);

    // 여러 제네릭 타입
    fn pair<T, U>(first: T, second: U) -> (T, U) {
        (first, second)
    }

    let p = pair(10, "hello");
    println!("  쌍: {:?}", p);

    // 제네릭 함수: 최대값 찾기
    fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
        let mut largest = list[0];
        for &item in list {
            if item > largest {
                largest = item;
            }
        }
        largest
    }

    let numbers = vec![34, 50, 25, 100, 65];
    println!("  최대값: {}", largest(&numbers));
}

// ============================================
// 제네릭 구조체
// ============================================
fn generic_structs() {
    // 제네릭 구조체
    struct Point<T> {
        x: T,
        y: T,
    }

    impl<T> Point<T> {
        fn new(x: T, y: T) -> Self {
            Point { x, y }
        }
    }

    // 특정 타입에 대한 메서드
    impl Point<f64> {
        fn distance_from_origin(&self) -> f64 {
            (self.x * self.x + self.y * self.y).sqrt()
        }
    }

    let integer_point = Point::new(5, 10);
    println!("  정수 점: ({}, {})", integer_point.x, integer_point.y);

    let float_point = Point::new(3.0, 4.0);
    println!("  실수 점: ({}, {})", float_point.x, float_point.y);
    println!("  원점으로부터 거리: {}", float_point.distance_from_origin());

    // 여러 제네릭 타입
    struct Pair<T, U> {
        first: T,
        second: U,
    }

    let pair = Pair {
        first: 42,
        second: "hello",
    };
    println!("  쌍: ({}, {})", pair.first, pair.second);
}

// ============================================
// 트레이트 정의 및 구현
// ============================================
fn trait_definitions() {
    // 트레이트 정의
    trait Drawable {
        fn draw(&self);
        fn area(&self) -> f64;
    }

    // 구조체 정의
    struct Circle {
        radius: f64,
    }

    struct Rectangle {
        width: f64,
        height: f64,
    }

    // 트레이트 구현
    impl Drawable for Circle {
        fn draw(&self) {
            println!("  원 그리기 (반지름: {})", self.radius);
        }

        fn area(&self) -> f64 {
            std::f64::consts::PI * self.radius * self.radius
        }
    }

    impl Drawable for Rectangle {
        fn draw(&self) {
            println!("  사각형 그리기 ({}x{})", self.width, self.height);
        }

        fn area(&self) -> f64 {
            self.width * self.height
        }
    }

    // 사용
    let circle = Circle { radius: 5.0 };
    circle.draw();
    println!("  원의 넓이: {}", circle.area());

    let rect = Rectangle {
        width: 10.0,
        height: 20.0,
    };
    rect.draw();
    println!("  사각형의 넓이: {}", rect.area());
}

// ============================================
// 트레이트 바운드
// ============================================
fn trait_bounds() {
    // 트레이트 바운드: T는 Display와 Clone을 구현해야 함
    fn print_and_clone<T: std::fmt::Display + Clone>(item: T) -> T {
        println!("  항목: {}", item);
        item.clone()
    }

    let value = print_and_clone(42);
    println!("  복제된 값: {}", value);

    // where 절 사용 (가독성 향상)
    fn complex_function<T, U>(t: T, u: U) -> String
    where
        T: std::fmt::Display + Clone,
        U: std::fmt::Display,
    {
        format!("{} and {}", t, u)
    }

    let result = complex_function(10, "hello");
    println!("  복잡한 함수 결과: {}", result);
}

// ============================================
// 트레이트 객체
// ============================================
fn trait_objects() {
    trait Processor {
        fn process(&self, input: &str) -> String;
    }

    struct UppercaseProcessor;
    struct LowercaseProcessor;

    impl Processor for UppercaseProcessor {
        fn process(&self, input: &str) -> String {
            input.to_uppercase()
        }
    }

    impl Processor for LowercaseProcessor {
        fn process(&self, input: &str) -> String {
            input.to_lowercase()
        }
    }

    // 트레이트 객체: 다양한 타입을 하나의 벡터에 저장
    let processors: Vec<Box<dyn Processor>> = vec![
        Box::new(UppercaseProcessor),
        Box::new(LowercaseProcessor),
    ];

    for processor in processors {
        let result = processor.process("Hello World");
        println!("  처리 결과: {}", result);
    }
}

// ============================================
// 실전 예제: AST Processor
// TypeScript의 제네릭 함수를 Rust로 변환
// ============================================
fn ast_processor_example() {
    // 트레이트 정의: AST 처리기
    trait AstProcessor {
        fn process(&self, module: &str) -> Result<String, String>;
    }

    // 구조체: Translation Processor
    struct TranslationProcessor {
        prefix: String,
    }

    impl AstProcessor for TranslationProcessor {
        fn process(&self, module: &str) -> Result<String, String> {
            Ok(format!("{}: {}", self.prefix, module))
        }
    }

    // 제네릭 함수: AST 처리
    fn process_ast<P: AstProcessor>(processor: &P, module: &str) -> Result<String, String> {
        processor.process(module)
    }

    // 사용
    let processor = TranslationProcessor {
        prefix: "translated".to_string(),
    };

    match process_ast(&processor, "module.ts") {
        Ok(result) => println!("  처리 결과: {}", result),
        Err(e) => println!("  에러: {}", e),
    }

    // 여러 프로세서를 트레이트 객체로 관리
    struct ValidationProcessor;

    impl AstProcessor for ValidationProcessor {
        fn process(&self, module: &str) -> Result<String, String> {
            if module.ends_with(".ts") {
                Ok(format!("유효한 모듈: {}", module))
            } else {
                Err("유효하지 않은 파일 확장자".to_string())
            }
        }
    }

    let processors: Vec<Box<dyn AstProcessor>> = vec![
        Box::new(TranslationProcessor {
            prefix: "translated".to_string(),
        }),
        Box::new(ValidationProcessor),
    ];

    for processor in processors {
        match processor.process("test.ts") {
            Ok(result) => println!("  {}", result),
            Err(e) => println!("  에러: {}", e),
        }
    }
}

// ============================================
// 제네릭과 트레이트 조합 예제
// ============================================
pub fn generic_trait_combination() {
    // 제네릭 트레이트
    trait Container<T> {
        fn add(&mut self, item: T);
        fn get(&self, index: usize) -> Option<&T>;
        fn len(&self) -> usize;
    }

    // Vec를 Container로 구현
    impl<T> Container<T> for Vec<T> {
        fn add(&mut self, item: T) {
            self.push(item);
        }

        fn get(&self, index: usize) -> Option<&T> {
            self.get(index)
        }

        fn len(&self) -> usize {
            self.len()
        }
    }

    // 제네릭 함수: Container를 사용
    fn process_container<C, T>(container: &C, index: usize) -> Option<String>
    where
        C: Container<T>,
        T: std::fmt::Display,
    {
        container.get(index).map(|item| format!("{}", item))
    }

    let mut vec: Vec<i32> = Vec::new();
    vec.add(10);
    vec.add(20);
    vec.add(30);

    if let Some(result) = process_container(&vec, 1) {
        println!("  컨테이너 항목: {}", result);
    }
}

