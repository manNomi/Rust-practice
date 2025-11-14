// ============================================
// 8. 정규표현식 (Regex) 학습
// TypeScript의 정규표현식을 Rust로 변환하는 방법
// ============================================

// 주의: 이 파일을 실행하려면 Cargo.toml에 regex 의존성이 필요합니다
// [dependencies]
// regex = "1"

pub fn run_regex_examples() {
    println!("\n=== 정규표현식 (Regex) 학습 ===\n");
    println!("이 예제는 regex crate가 필요합니다.");
    println!("Cargo.toml에 다음을 추가하세요:");
    println!("  [dependencies]");
    println!("  regex = \"1\"\n");
}

// 실제 사용 예제 (regex crate가 있을 때만 컴파일)
#[cfg(feature = "regex")]
pub fn regex_examples() {
    use regex::Regex;

    println!("=== 정규표현식 예제 ===\n");

    // ============================================
    // 1. 기본 매칭
    // ============================================
    println!("1. 기본 매칭");
    basic_matching();
    println!();

    // ============================================
    // 2. 캡처 그룹
    // ============================================
    println!("2. 캡처 그룹");
    capture_groups();
    println!();

    // ============================================
    // 3. 실전 예제: 상수 이름 검증
    // ============================================
    println!("3. 실전 예제: 상수 이름 검증");
    constant_name_validation();
    println!();

    // ============================================
    // 4. 문자열 치환
    // ============================================
    println!("4. 문자열 치환");
    string_replacement();
    println!();
}

// ============================================
// TypeScript 코드:
// if (/^[A-Z][A-Z0-9_]*$/.test(varName)) {
//   // ALL_CAPS 패턴
// }
// ============================================
#[cfg(feature = "regex")]
fn basic_matching() {
    use regex::Regex;

    // 정규식 컴파일
    let re = Regex::new(r"^[A-Z][A-Z0-9_]*$").unwrap();

    let test_cases = vec!["CONSTANT", "CONSTANT_1", "constant", "CONSTANT-1", "1CONSTANT"];

    for var_name in test_cases {
        if re.is_match(var_name) {
            println!("  '{}'는 유효한 상수 이름입니다", var_name);
        } else {
            println!("  '{}'는 유효하지 않은 상수 이름입니다", var_name);
        }
    }

    // find(): 첫 번째 매치 찾기
    let text = "The year is 2024 and the month is 12";
    let year_re = Regex::new(r"\d{4}").unwrap();
    if let Some(mat) = year_re.find(text) {
        println!("  첫 번째 숫자: {}", mat.as_str());
    }

    // find_iter(): 모든 매치 찾기
    println!("  모든 숫자:");
    for mat in year_re.find_iter(text) {
        println!("    {}", mat.as_str());
    }
}

// ============================================
// 캡처 그룹: 매치된 부분 추출
// ============================================
#[cfg(feature = "regex")]
fn capture_groups() {
    use regex::Regex;

    // 이메일 주소 파싱
    let email_re = Regex::new(r"^([a-zA-Z0-9._%+-]+)@([a-zA-Z0-9.-]+\.[a-zA-Z]{2,})$").unwrap();

    let email = "user@example.com";
    if let Some(caps) = email_re.captures(email) {
        println!("  전체 매치: {}", &caps[0]);
        println!("  사용자명: {}", &caps[1]);
        println!("  도메인: {}", &caps[2]);
    }

    // 날짜 파싱
    let date_re = Regex::new(r"(\d{4})-(\d{2})-(\d{2})").unwrap();
    let date = "2024-12-25";
    
    if let Some(caps) = date_re.captures(date) {
        println!("  날짜: {}-{}-{}", &caps[1], &caps[2], &caps[3]);
    }

    // 이름이 있는 캡처 그룹
    let named_re = Regex::new(r"(?P<year>\d{4})-(?P<month>\d{2})-(?P<day>\d{2})").unwrap();
    if let Some(caps) = named_re.captures(date) {
        println!("  연도: {}", &caps["year"]);
        println!("  월: {}", &caps["month"]);
        println!("  일: {}", &caps["day"]);
    }
}

// ============================================
// 실전 예제: 상수 이름 검증
// TypeScript:
// if (/^[A-Z][A-Z0-9_]*$/.test(varName)) {
//   return true;
// }
// ============================================
#[cfg(feature = "regex")]
fn constant_name_validation() {
    use regex::Regex;

    fn is_valid_constant_name(name: &str) -> bool {
        let re = Regex::new(r"^[A-Z][A-Z0-9_]*$").unwrap();
        re.is_match(name)
    }

    let test_names = vec![
        "CONSTANT",
        "CONSTANT_1",
        "MY_CONSTANT",
        "constant",      // 소문자로 시작
        "CONSTANT-1",   // 하이픈 포함
        "1CONSTANT",    // 숫자로 시작
        "CONSTANT ",    // 공백 포함
    ];

    for name in test_names {
        let valid = is_valid_constant_name(name);
        println!("  '{}': {}", name, if valid { "유효" } else { "무효" });
    }
}

// ============================================
// 문자열 치환
// ============================================
#[cfg(feature = "regex")]
fn string_replacement() {
    use regex::Regex;

    // 단순 치환
    let re = Regex::new(r"cat").unwrap();
    let text = "I have a cat and a cat";
    let replaced = re.replace_all(text, "dog");
    println!("  원본: {}", text);
    println!("  치환: {}", replaced);

    // 캡처 그룹을 사용한 치환
    let re = Regex::new(r"(\d{4})-(\d{2})-(\d{2})").unwrap();
    let date = "2024-12-25";
    let replaced = re.replace(date, "$3/$2/$1"); // DD/MM/YYYY 형식으로 변경
    println!("  날짜 형식 변경: {} -> {}", date, replaced);

    // 클로저를 사용한 치환
    let re = Regex::new(r"\d+").unwrap();
    let text = "I have 3 apples and 5 oranges";
    let replaced = re.replace_all(text, |caps: &regex::Captures| {
        let num: i32 = caps[0].parse().unwrap();
        (num * 2).to_string()
    });
    println!("  숫자 2배: {} -> {}", text, replaced);
}

// ============================================
// 컴파일 타임 정규식 (성능 최적화)
// ============================================
#[cfg(feature = "regex")]
pub fn compile_time_regex() {
    use regex::Regex;
    use once_cell::sync::Lazy;

    // Lazy static 정규식 (한 번만 컴파일)
    static CONSTANT_NAME_RE: Lazy<Regex> = Lazy::new(|| {
        Regex::new(r"^[A-Z][A-Z0-9_]*$").unwrap()
    });

    let test_name = "MY_CONSTANT";
    if CONSTANT_NAME_RE.is_match(test_name) {
        println!("  '{}'는 유효한 상수 이름입니다", test_name);
    }
}

// ============================================
// 정규식 없이 간단한 패턴 매칭
// ============================================
pub fn simple_pattern_matching() {
    println!("정규식 없이 간단한 패턴 매칭:\n");

    // starts_with, ends_with
    let text = "CONSTANT_NAME";
    if text.starts_with("CONSTANT") {
        println!("  '{}'는 'CONSTANT'로 시작합니다", text);
    }

    if text.ends_with("NAME") {
        println!("  '{}'는 'NAME'으로 끝납니다", text);
    }

    // contains
    if text.contains("_") {
        println!("  '{}'는 언더스코어를 포함합니다", text);
    }

    // chars()로 문자 단위 검사
    fn is_all_caps(s: &str) -> bool {
        s.chars().all(|c| c.is_uppercase() || c.is_numeric() || c == '_')
    }

    let test_cases = vec!["CONSTANT", "constant", "CONSTANT_1"];
    for case in test_cases {
        println!("  '{}'는 모두 대문자/숫자/언더스코어: {}", 
                 case, is_all_caps(case));
    }
}

