// ============================================
// Rust 마이그레이션 학습 메인 실행 파일
// 모든 학습 예제를 실행하는 통합 파일
// ============================================

// 각 학습 모듈 import
mod collections;
mod error_handling;
mod async_examples;
mod file_io;
mod regex_examples;
mod swc_examples;
mod cli_examples;
mod generics_traits;

// 모듈에서 함수 사용
use collections::run_collections_examples;
use error_handling::run_error_handling_examples;
use async_examples::run_async_examples;
use file_io::run_file_io_examples;
use regex_examples::run_regex_examples;
use swc_examples::run_swc_examples;
use cli_examples::run_cli_examples;
use generics_traits::run_generics_traits_examples;

fn main() {
    println!("╔════════════════════════════════════════════════════════════╗");
    println!("║     Rust 마이그레이션 학습 가이드 - 통합 실행              ║");
    println!("╚════════════════════════════════════════════════════════════╝\n");

    // 기본 학습 내용 (의존성 불필요)
    println!("=== 기본 학습 내용 (의존성 불필요) ===\n");

    // 1. 컬렉션 (HashMap, HashSet)
    run_collections_examples();

    // 2. 에러 처리 (Result & Option)
    run_error_handling_examples();

    // 3. 파일 I/O & 경로 처리
    run_file_io_examples();

    // 4. 제네릭 & 트레이트
    run_generics_traits_examples();

    // 고급 학습 내용 (의존성 필요)
    println!("\n=== 고급 학습 내용 (의존성 필요) ===\n");

    // 5. 비동기 프로그래밍
    run_async_examples();

    // 6. 정규표현식
    run_regex_examples();

    // 7. SWC Rust API
    run_swc_examples();

    // 8. CLI 도구 개발
    run_cli_examples();

    println!("\n╔════════════════════════════════════════════════════════════╗");
    println!("║                    학습 완료!                             ║");
    println!("╚════════════════════════════════════════════════════════════╝\n");

    println!("다음 단계:");
    println!("1. Cargo.toml에 필요한 의존성 추가");
    println!("2. 각 예제 파일을 개별적으로 실행해보기");
    println!("3. 예제 코드를 수정하며 실습하기");
    println!("4. 실제 프로젝트에 적용하기\n");
}

