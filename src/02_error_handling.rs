// ============================================
// 3. 에러 처리 (Result & Option) 심화 학습
// TypeScript의 try-catch를 Rust로 변환하는 방법
// ============================================

use std::fs;
use std::io;

pub fn run_error_handling_examples() {
    println!("\n=== 에러 처리 (Result & Option) 심화 학습 ===\n");

    // ============================================
    // 1. Result 기본 사용법
    // ============================================
    println!("1. Result 기본 사용법");
    result_basics();
    println!();

    // ============================================
    // 2. ? 연산자 (에러 전파)
    // ============================================
    println!("2. ? 연산자 (에러 전파)");
    question_mark_operator();
    println!();

    // ============================================
    // 3. Option 기본 사용법
    // ============================================
    println!("3. Option 기본 사용법");
    option_basics();
    println!();

    // ============================================
    // 4. match vs if let vs unwrap
    // ============================================
    println!("4. match vs if let vs unwrap");
    error_handling_patterns();
    println!();

    // ============================================
    // 5. 커스텀 에러 타입
    // ============================================
    println!("5. 커스텀 에러 타입");
    custom_error_types();
    println!();

    // ============================================
    // 6. 실전 예제: 파일 읽기
    // ============================================
    println!("6. 실전 예제: 파일 읽기");
    file_reading_example();
    println!();
}

// ============================================
// TypeScript 코드:
// try {
//   const result = riskyOperation();
//   console.log(result);
// } catch (error) {
//   console.error(error);
// }
// ============================================
fn result_basics() {
    // Result<T, E>: 성공(Ok) 또는 실패(Err)
    fn parse_number(s: &str) -> Result<i32, String> {
        s.parse::<i32>()
            .map_err(|_| format!("'{}'는 숫자가 아닙니다", s))
    }

    // match로 처리
    match parse_number("42") {
        Ok(value) => println!("  성공: {}", value),
        Err(e) => println!("  에러: {}", e),
    }

    match parse_number("abc") {
        Ok(value) => println!("  성공: {}", value),
        Err(e) => println!("  에러: {}", e),
    }

    // unwrap(): 에러면 panic (개발 중에만 사용)
    let value = parse_number("100").unwrap();
    println!("  unwrap() 결과: {}", value);

    // unwrap_or(): 에러면 기본값 반환
    let value = parse_number("invalid").unwrap_or(0);
    println!("  unwrap_or(0) 결과: {}", value);

    // expect(): 에러면 panic + 메시지
    let value = parse_number("200").expect("숫자 파싱 실패");
    println!("  expect() 결과: {}", value);
}

// ============================================
// ? 연산자: 에러를 자동으로 전파
// TypeScript의 async/await와 유사한 패턴
// ============================================
fn question_mark_operator() {
    // ? 연산자: Result가 Err면 즉시 반환, Ok면 값을 추출
    fn read_and_parse() -> Result<i32, String> {
        // 여러 단계의 에러를 자동으로 전파
        let num1 = parse_number("10")?; // ? 연산자 사용
        let num2 = parse_number("20")?;
        Ok(num1 + num2)
    }

    match read_and_parse() {
        Ok(sum) => println!("  합계: {}", sum),
        Err(e) => println!("  에러: {}", e),
    }

    // 중첩된 함수 호출
    fn complex_operation() -> Result<String, String> {
        let a = parse_number("5")?;
        let b = parse_number("3")?;
        let result = a * b;
        Ok(format!("결과: {}", result))
    }

    match complex_operation() {
        Ok(msg) => println!("  {}", msg),
        Err(e) => println!("  에러: {}", e),
    }
}

fn parse_number(s: &str) -> Result<i32, String> {
    s.parse::<i32>()
        .map_err(|_| format!("'{}'는 숫자가 아닙니다", s))
}

// ============================================
// Option<T>: 값이 있을 수도, 없을 수도 있음
// TypeScript의 null/undefined와 유사
// ============================================
fn option_basics() {
    // Option::Some(value) 또는 Option::None
    fn find_index(arr: &[i32], target: i32) -> Option<usize> {
        arr.iter().position(|&x| x == target)
    }

    let numbers = vec![1, 2, 3, 4, 5];

    match find_index(&numbers, 3) {
        Some(index) => println!("  찾음: 인덱스 {}", index),
        None => println!("  없음"),
    }

    // if let 패턴
    if let Some(index) = find_index(&numbers, 5) {
        println!("  if let: 인덱스 {}", index);
    }

    // unwrap_or()
    let index = find_index(&numbers, 10).unwrap_or(0);
    println!("  unwrap_or(0): {}", index);

    // map(): Option 내부 값 변환
    let doubled = find_index(&numbers, 3).map(|i| i * 2);
    println!("  map: {:?}", doubled);

    // and_then(): 체이닝
    let result = find_index(&numbers, 3)
        .and_then(|i| Some(i * 10));
    println!("  and_then: {:?}", result);
}

// ============================================
// 에러 처리 패턴 비교
// ============================================
fn error_handling_patterns() {
    let result: Result<i32, String> = Ok(42);

    // 1. match: 모든 경우 처리 (가장 명시적)
    println!("  match 패턴:");
    match result {
        Ok(value) => println!("    값: {}", value),
        Err(e) => println!("    에러: {}", e),
    }

    // 2. if let: 하나의 경우만 처리
    println!("  if let 패턴:");
    if let Ok(value) = result {
        println!("    값: {}", value);
    }

    // 3. unwrap(): 개발 중에만 사용 (프로덕션 금지)
    let value = result.unwrap();
    println!("  unwrap(): {}", value);

    // 4. unwrap_or(): 기본값 제공
    let error_result: Result<i32, String> = Err("에러".to_string());
    let value = error_result.unwrap_or(0);
    println!("  unwrap_or(0): {}", value);

    // 5. map_err(): 에러 타입 변환
    let result = error_result.map_err(|e| format!("변환된 에러: {}", e));
    println!("  map_err(): {:?}", result);
}

// ============================================
// 커스텀 에러 타입
// ============================================
#[derive(Debug)]
enum ParseError {
    InvalidNumber(String),
    EmptyInput,
    OutOfRange,
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ParseError::InvalidNumber(s) => write!(f, "잘못된 숫자: {}", s),
            ParseError::EmptyInput => write!(f, "입력이 비어있습니다"),
            ParseError::OutOfRange => write!(f, "범위를 벗어났습니다"),
        }
    }
}

impl std::error::Error for ParseError {}

fn custom_error_types() {
    fn parse_positive_number(s: &str) -> Result<i32, ParseError> {
        if s.is_empty() {
            return Err(ParseError::EmptyInput);
        }

        let num = s.parse::<i32>()
            .map_err(|_| ParseError::InvalidNumber(s.to_string()))?;

        if num < 0 {
            return Err(ParseError::OutOfRange);
        }

        Ok(num)
    }

    // 다양한 에러 케이스 테스트
    let test_cases = vec!["42", "", "abc", "-10"];

    for input in test_cases {
        match parse_positive_number(input) {
            Ok(value) => println!("  '{}' -> {}", input, value),
            Err(e) => println!("  '{}' -> 에러: {}", input, e),
        }
    }
}

// ============================================
// 실전 예제: 파일 읽기
// TypeScript:
// try {
//   const code = fs.readFileSync(filePath, "utf-8");
//   const ast = parseFile(code);
// } catch (error) {
//   console.error(`Error: ${error}`);
// }
// ============================================
fn file_reading_example() {
    // 파일 읽기 함수
    fn read_file_safe(path: &str) -> Result<String, io::Error> {
        fs::read_to_string(path)
    }

    // match로 처리
    match read_file_safe("nonexistent.txt") {
        Ok(content) => println!("  파일 내용: {}", content),
        Err(e) => println!("  파일 읽기 실패: {}", e),
    }

    // ? 연산자로 체이닝
    fn read_and_process() -> Result<String, io::Error> {
        let content = read_file_safe("test.txt")?;
        Ok(format!("처리된 내용: {}", content.len()))
    }

    match read_and_process() {
        Ok(result) => println!("  {}", result),
        Err(e) => println!("  에러: {}", e),
    }

    // unwrap_or_else: 에러 발생 시 클로저 실행
    let content = read_file_safe("test.txt")
        .unwrap_or_else(|_| "기본 내용".to_string());
    println!("  unwrap_or_else: {}", content);
}

// ============================================
// Result와 Option 변환
// ============================================
pub fn result_option_conversion() {
    // Option을 Result로 변환
    let option: Option<i32> = Some(42);
    let result: Result<i32, String> = option.ok_or("값이 없습니다".to_string());
    println!("Option -> Result: {:?}", result);

    // Result를 Option으로 변환
    let result: Result<i32, String> = Ok(42);
    let option = result.ok();
    println!("Result -> Option: {:?}", option);
}

