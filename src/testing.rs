// ============================================
// Rust 테스트 코드 작성법 학습
// TypeScript의 Jest/Mocha를 Rust로 변환하는 방법
// ============================================

pub fn run_testing_examples() {
    println!("\n=== Rust 테스트 코드 작성법 학습 ===\n");

    println!("Rust는 내장 테스트 프레임워크를 제공합니다.");
    println!("별도의 테스트 라이브러리가 필요 없습니다!\n");

    println!("테스트 실행 방법:");
    println!("  cargo test              - 모든 테스트 실행");
    println!("  cargo test --lib        - 라이브러리 테스트만 실행");
    println!("  cargo test --bin main   - 바이너리 테스트만 실행");
    println!("  cargo test test_name    - 특정 테스트만 실행");
    println!("  cargo test -- --nocapture - 출력 표시\n");

    println!("TypeScript vs Rust 테스트 비교:");
    println!("  TypeScript (Jest):");
    println!("    describe('add', () => {{");
    println!("      it('should add two numbers', () => {{");
    println!("        expect(add(2, 3)).toBe(5);");
    println!("      }});");
    println!("    }});\n");

    println!("  Rust:");
    println!("    #[test]");
    println!("    fn test_add() {{");
    println!("        assert_eq!(add(2, 3), 5);");
    println!("    }}\n");
}

// ============================================
// 테스트 가능한 공개 함수들
// ============================================

pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

pub fn subtract(a: i32, b: i32) -> i32 {
    a - b
}

pub fn multiply(a: i32, b: i32) -> i32 {
    a * b
}

pub fn is_even(n: i32) -> bool {
    n % 2 == 0
}

pub fn divide_safe(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err("0으로 나눌 수 없습니다".to_string())
    } else {
        Ok(a / b)
    }
}

// ============================================
// 단위 테스트 모듈
// ============================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(2, 3), 5);
        assert_eq!(add(0, 0), 0);
        assert_eq!(add(-1, 1), 0);
    }

    #[test]
    fn test_subtract() {
        assert_eq!(subtract(5, 3), 2);
        assert_eq!(subtract(0, 0), 0);
        assert_eq!(subtract(1, 3), -2);
    }

    #[test]
    fn test_multiply() {
        assert_eq!(multiply(2, 3), 6);
        assert_eq!(multiply(0, 100), 0);
        assert_ne!(multiply(2, 3), 5);
    }

    #[test]
    fn test_is_even() {
        assert!(is_even(2));
        assert!(is_even(0));
        assert!(!is_even(1));
        assert!(!is_even(3));
    }

    #[test]
    fn test_divide_safe() {
        assert!(divide_safe(10.0, 2.0).is_ok());
        assert_eq!(divide_safe(10.0, 2.0).unwrap(), 5.0);
        assert!(divide_safe(10.0, 0.0).is_err());
        assert_eq!(divide_safe(10.0, 0.0).unwrap_err(), "0으로 나눌 수 없습니다");
    }

    #[test]
    fn test_result() -> Result<(), String> {
        let result = divide_safe(10.0, 2.0)?;
        assert_eq!(result, 5.0);
        Ok(())
    }

    #[test]
    #[should_panic]
    fn test_panic() {
        panic!("이 테스트는 panic을 발생시킵니다");
    }

    // 커스텀 실패 메시지
    #[test]
    fn test_with_message() {
        let result = add(2, 3);
        assert_eq!(result, 5, "2 + 3은 5여야 합니다. 실제 값: {}", result);
    }
}
