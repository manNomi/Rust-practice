// ============================================
// TypeScript/JavaScript 개발자를 위한 Rust
// 주요 차이점 학습 가이드
// ============================================

fn main() {
    println!("\n=== 1. 변수 & 타입 (Variables & Types) ===\n");
    variables_and_types();

    println!("\n=== 2. 불변성 (Immutability) ===\n");
    immutability();

    println!("\n=== 3. 소유권 (Ownership) ===\n");
    ownership();

    println!("\n=== 4. 빌림 & 참조 (Borrowing & References) ===\n");
    borrowing_and_references();

    println!("\n=== 5. 함수 & 반환값 (Functions & Return Values) ===\n");
    functions_and_return_values();

    println!("\n=== 6. 에러 처리 (Error Handling) ===\n");
    error_handling();

    println!("\n=== 7. 패턴 매칭 (Pattern Matching) ===\n");
    pattern_matching();

    println!("\n=== 8. 구조체 (Structs) ===\n");
    structs_example();
}

// ============================================
// 1. 변수 & 타입
// ============================================
fn variables_and_types() {
    // TS: let x: number = 5;
    // Rust는 타입을 명시하거나 추론할 수 있음
    let x: i32 = 5;
    println!("x (i32): {}", x);

    // TS: const y = 10;
    // Rust는 기본적으로 불변이지만 명시적으로 const를 사용할 수도 있음
    const Y: i32 = 10;
    println!("Y (const i32): {}", Y);

    // 타입 추론
    let z = 15; // Rust가 i32로 추론
    println!("z (inferred): {}", z);

    // 정수 타입들
    let byte: u8 = 255; // unsigned 8-bit
    let small: i16 = -32768; // signed 16-bit
    let float: f64 = 3.14;
    println!("byte: {}, small: {}, float: {}", byte, small, float);

    // 문자열 (중요한 차이!)
    // TS: const str: string = "hello"; // 가비지 콜렉션 됨
    let str1: &str = "hello"; // 문자 리터럴 (스택에 저장)
    println!("str1 (literal): {}", str1);

    let str2: String = String::from("hello"); // 힙에 할당되는 String
    println!("str2 (String): {}", str2);
}

// ============================================
// 2. 불변성 (Immutability)
// ============================================
fn immutability() {
    // TS: let x = 5; x = 10; // 가능
    let x = 5;
    println!("x (immutable): {}", x);

    // TS처럼 하려면 명시적으로 mut을 사용해야 함
    // Rust의 철학: 기본은 불변, 필요할 때만 가변
    let mut y = 5;
    println!("y (before): {}", y);
    y = 10;
    println!("y (after): {}", y);

    // const vs let
    // const는 컴파일 타임에 결정되어야 함 (TS와 비슷)
    const MAX_POINTS: u32 = 100_000;
    println!("MAX_POINTS (const): {}", MAX_POINTS);
}

// ============================================
// 3. 소유권 (Ownership) - Rust의 가장 큰 특징!
// ============================================
fn ownership() {
    // TS: 가비지 콜렉션으로 메모리 자동 관리
    // Rust: 소유권 규칙으로 메모리 관리 (컴파일 타임에 확인)

    let s1 = String::from("hello");
    println!("s1: {}", s1);

    // 이 순간 s1의 소유권이 s2로 이동 (move)
    let s2 = s1;
    println!("s2: {}", s2);

    // ❌ 이 코드는 컴파일 에러!
    // println!("s1: {}", s1); // s1은 더 이상 유효하지 않음

    // 복사하려면 clone을 사용
    let s3 = String::from("world");
    let s4 = s3.clone(); // 명시적 복사
    println!("s3: {}, s4: {}", s3, s4); // 둘 다 유효

    // 스택에만 있는 타입은 자동으로 복사됨
    let x = 5;
    let y = x; // Copy 트레이트가 구현되어 있음
    println!("x: {}, y: {} (both valid)", x, y);
}

// ============================================
// 4. 빌림 & 참조 (Borrowing & References)
// ============================================
fn borrowing_and_references() {
    // 불변 참조 (읽기만 가능)
    let s1 = String::from("hello");
    let len = calculate_length(&s1); // 참조 전달
    println!("'{}' has length {}", s1, len);
    // s1은 여전히 유효함!

    // 가변 참조 (수정 가능)
    let mut s2 = String::from("world");
    modify_string(&mut s2); // 가변 참조
    println!("modified s2: {}", s2);

    // 중요: 한 번에 하나의 가변 참조만 가능
    let mut x = 5;
    let r1 = &mut x;
    // ❌ let r2 = &mut x; // 컴파일 에러!
    *r1 = 10; // * 연산자로 역참조
    println!("x after modification: {}", x);
}

fn calculate_length(s: &String) -> usize {
    s.len()
    // s는 함수를 벗어나면서 버려짐, 하지만 String은 여전히 유효
}

fn modify_string(s: &mut String) {
    s.push_str(" modified");
}

// ============================================
// 5. 함수 & 반환값 (Functions & Return Values)
// ============================================
fn functions_and_return_values() {
    // TS: function add(a: number, b: number): number { return a + b; }
    // Rust는 return을 생략할 수 있음 (세미콜론 없는 표현식이 반환됨)

    let result1 = add(5, 3);
    println!("add(5, 3) = {}", result1);

    // 여러 값 반환은 튜플 사용
    let (sum, product) = calculate_both(4, 5);
    println!("sum: {}, product: {}", sum, product);

    // 함수 포인터 (TS의 함수형 프로그래밍과 유사)
    let op = add; // 함수를 변수에 할당
    println!("op(10, 20) = {}", op(10, 20));
}

// return 키워드 없음, 마지막 표현식이 반환됨
fn add(a: i32, b: i32) -> i32 {
    a + b // 세미콜론 없음!
}

fn calculate_both(a: i32, b: i32) -> (i32, i32) {
    (a + b, a * b) // 튜플 반환
}

// ============================================
// 6. 에러 처리 (Error Handling)
// ============================================
fn error_handling() {
    // TS: try-catch 또는 Optional chaining
    // Rust: Result & Option 타입 (컴파일 타임에 강제)

    // Option: 값이 있을 수도, 없을 수도 있음
    let maybe_number: Option<i32> = Some(5);
    match maybe_number {
        Some(num) => println!("Got a number: {}", num),
        None => println!("No number"),
    }

    // Result: 성공 또는 실패
    let result: Result<i32, String> = Ok(10);
    match result {
        Ok(val) => println!("Success: {}", val),
        Err(e) => println!("Error: {}", e),
    }

    // 간단한 방법: unwrap (TS의 Non-null Assertion과 유사)
    let value = Some(42).unwrap();
    println!("Unwrapped value: {}", value);

    // 안전한 방법: unwrap_or (기본값 제공)
    let value2 = None.unwrap_or(0);
    println!("Unwrapped with default: {}", value2);
}

// ============================================
// 7. 패턴 매칭 (Pattern Matching)
// ============================================
fn pattern_matching() {
    // TS의 switch보다 훨씬 강력함
    let number = 3;

    match number {
        1 => println!("One"),
        2 | 3 => println!("Two or Three"), // OR 패턴
        4..=6 => println!("Four to Six"), // 범위
        _ => println!("Other"), // 나머지
    }

    // 열거형과 함께 사용
    let result: Result<i32, &str> = Ok(100);
    match result {
        Ok(value) if value > 50 => println!("Big value: {}", value),
        Ok(value) => println!("Small value: {}", value),
        Err(e) => println!("Error: {}", e),
    }
}
// 1) Result + ? 를 활용한 에러 전파
fn read_number() -> Result<i32, String> {
    let val = Some(10).ok_or("No value")?;   // 실패하면 Err로 반환됨
    Ok(val + 1)
}

use std::panic;

// fn main() {
//     let result = panic::catch_unwind(|| {
//         panic!("Boom!");
//     });

//     match result {
//         Ok(_) => println!("No panic"),
//         Err(_) => println!("Caught a panic"),
//     }
// }

// ✅ Why no try-catch?

// Rust 철학:

// 에러는 타입 시스템으로 처리해라
// (= 컴파일러가 확인할 수 있게)

// 그래서
// ✅ 어디서 에러 날지 명확
// ✅ 핸들링을 강제 → 안전
// ❌ try-catch만큼 즉흥적이지 않음

// ✅ Rust의 Result::Err(...) 는 “에러를 반환(return) 하는 것”이지
// JS/TS처럼 throw 하는 게 아니다.

// 즉,
// 	•	Err → 값으로 리턴
// 	•	throw → 스택 던지기(예외)

// Rust는 후자를 기본적으로 안 한다.

// ============================================
// 8. 구조체 (Structs)
// ============================================
fn structs_example() {
    // TS의 인터페이스나 타입과 유사하지만 구조체는 메서드도 가질 수 있음
    struct Person {
        name: String,
        age: u32,
    }

    let person = Person {
        name: String::from("Alice"),
        age: 30,
    };

    println!("Name: {}, Age: {}", person.name, person.age);

    // 메서드는 impl 블록으로 구현
    impl Person {
        fn greet(&self) {
            println!("Hi, I'm {} and I'm {} years old", self.name, self.age);
        }
    }

    person.greet();

    // 튜플 구조체 (TS에는 없는 개념)
    struct Color(u8, u8, u8);
    let color = Color(255, 128, 0);
    println!("Color: RGB({}, {}, {})", color.0, color.1, color.2);
}
