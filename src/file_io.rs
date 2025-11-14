// ============================================
// 7. 파일 I/O & 경로 처리 학습
// TypeScript의 fs, path를 Rust로 변환하는 방법
// ============================================

use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};

pub fn run_file_io_examples() {
    println!("\n=== 파일 I/O & 경로 처리 학습 ===\n");

    // ============================================
    // 1. 파일 읽기/쓰기
    // ============================================
    println!("1. 파일 읽기/쓰기");
    file_read_write();
    println!();

    // ============================================
    // 2. 경로 처리
    // ============================================
    println!("2. 경로 처리");
    path_handling();
    println!();

    // ============================================
    // 3. 디렉토리 작업
    // ============================================
    println!("3. 디렉토리 작업");
    directory_operations();
    println!();

    // ============================================
    // 4. 실전 예제: 파일 처리
    // ============================================
    println!("4. 실전 예제: 파일 처리");
    file_processing_example();
    println!();
}

// ============================================
// TypeScript 코드:
// import * as fs from "fs";
// const code = fs.readFileSync(filePath, "utf-8");
// fs.writeFileSync(filePath, output.code, "utf-8");
// ============================================
fn file_read_write() {
    let file_path = "test_output.txt";
    let content = "Hello, Rust!";

    // 파일 쓰기
    match fs::write(file_path, content) {
        Ok(_) => println!("  파일 쓰기 성공: {}", file_path),
        Err(e) => println!("  파일 쓰기 실패: {}", e),
    }

    // 파일 읽기
    match fs::read_to_string(file_path) {
        Ok(content) => println!("  파일 내용: {}", content),
        Err(e) => println!("  파일 읽기 실패: {}", e),
    }

    // 바이너리 읽기
    match fs::read(file_path) {
        Ok(bytes) => println!("  바이너리 크기: {} bytes", bytes.len()),
        Err(e) => println!("  바이너리 읽기 실패: {}", e),
    }

    // 파일 존재 확인
    if Path::new(file_path).exists() {
        println!("  파일이 존재합니다: {}", file_path);
    }

    // 파일 메타데이터
    match fs::metadata(file_path) {
        Ok(metadata) => {
            println!("  파일 크기: {} bytes", metadata.len());
            println!("  읽기 전용: {}", metadata.permissions().readonly());
        }
        Err(e) => println!("  메타데이터 읽기 실패: {}", e),
    }
}

// ============================================
// TypeScript 코드:
// import * as path from "path";
// const resolvedPath = path.resolve(currentDir, importPath);
// const ext = path.extname(filePath);
// ============================================
fn path_handling() {
    // 경로 생성
    let path = Path::new("/home/user/documents/file.txt");
    println!("  원본 경로: {:?}", path);

    // 경로 조작
    println!("  부모 디렉토리: {:?}", path.parent());
    println!("  파일명: {:?}", path.file_name());
    println!("  확장자: {:?}", path.extension());
    println!("  스템 (확장자 제외): {:?}", path.file_stem());

    // 경로 결합
    let base = Path::new("/home/user");
    let joined = base.join("documents").join("file.txt");
    println!("  결합된 경로: {:?}", joined);

    // PathBuf 사용 (가변 경로)
    let mut path_buf = PathBuf::from("/home/user");
    path_buf.push("documents");
    path_buf.push("file.txt");
    println!("  PathBuf: {:?}", path_buf);

    // 경로 정규화
    let normalized = path_buf.canonicalize();
    match normalized {
        Ok(path) => println!("  정규화된 경로: {:?}", path),
        Err(_) => println!("  경로 정규화 실패 (파일이 존재하지 않음)"),
    }

    // 경로 분해
    let components: Vec<_> = path.iter().collect();
    println!("  경로 구성 요소: {:?}", components);

    // 절대 경로 vs 상대 경로
    let abs_path = Path::new("/absolute/path");
    let rel_path = Path::new("./relative/path");
    println!("  절대 경로: {} (is_absolute: {})", 
             abs_path.display(), abs_path.is_absolute());
    println!("  상대 경로: {} (is_absolute: {})", 
             rel_path.display(), rel_path.is_absolute());
}

// ============================================
// 디렉토리 작업
// ============================================
fn directory_operations() {
    let dir_path = "test_dir";

    // 디렉토리 생성
    match fs::create_dir(dir_path) {
        Ok(_) => println!("  디렉토리 생성 성공: {}", dir_path),
        Err(e) => {
            if e.kind() == io::ErrorKind::AlreadyExists {
                println!("  디렉토리가 이미 존재합니다: {}", dir_path);
            } else {
                println!("  디렉토리 생성 실패: {}", e);
            }
        }
    }

    // 재귀적 디렉토리 생성
    let nested_dir = "test_dir/nested/deep";
    match fs::create_dir_all(nested_dir) {
        Ok(_) => println!("  중첩 디렉토리 생성 성공: {}", nested_dir),
        Err(e) => println!("  중첩 디렉토리 생성 실패: {}", e),
    }

    // 디렉토리 읽기
    match fs::read_dir(dir_path) {
        Ok(entries) => {
            println!("  디렉토리 내용:");
            for entry in entries {
                match entry {
                    Ok(entry) => {
                        let path = entry.path();
                        let file_type = entry.file_type().unwrap();
                        if file_type.is_dir() {
                            println!("    [DIR]  {:?}", path);
                        } else {
                            println!("    [FILE] {:?}", path);
                        }
                    }
                    Err(e) => println!("    에러: {}", e),
                }
            }
        }
        Err(e) => println!("  디렉토리 읽기 실패: {}", e),
    }

    // 디렉토리 삭제
    match fs::remove_dir_all(dir_path) {
        Ok(_) => println!("  디렉토리 삭제 성공: {}", dir_path),
        Err(e) => println!("  디렉토리 삭제 실패: {}", e),
    }
}

// ============================================
// 실전 예제: 파일 처리
// TypeScript:
// const code = fs.readFileSync(filePath, "utf-8");
// const resolvedPath = path.resolve(currentDir, importPath);
// fs.writeFileSync(filePath, output.code, "utf-8");
// ============================================
fn file_processing_example() {
    // 파일 읽기 및 처리
    fn process_file(file_path: &Path) -> Result<String, io::Error> {
        let code = fs::read_to_string(file_path)?;
        
        // 간단한 처리 (예: 대문자 변환)
        let processed = code.to_uppercase();
        
        Ok(processed)
    }

    // 경로 해석
    fn resolve_path(current_dir: &str, import_path: &str) -> PathBuf {
        let base = Path::new(current_dir);
        base.join(import_path)
    }

    // 테스트 파일 생성
    let test_file = "test_process.txt";
    fs::write(test_file, "hello rust").unwrap();

    // 파일 처리
    match process_file(Path::new(test_file)) {
        Ok(processed) => {
            println!("  처리된 내용: {}", processed);
            
            // 처리된 내용을 새 파일에 저장
            let output_file = "test_output_processed.txt";
            match fs::write(output_file, processed) {
                Ok(_) => println!("  출력 파일 저장 성공: {}", output_file),
                Err(e) => println!("  출력 파일 저장 실패: {}", e),
            }
        }
        Err(e) => println!("  파일 처리 실패: {}", e),
    }

    // 경로 해석 예제
    let current_dir = "/home/user/project";
    let import_path = "./src/utils";
    let resolved = resolve_path(current_dir, import_path);
    println!("  경로 해석: {} + {} = {:?}", 
             current_dir, import_path, resolved);

    // 정리
    let _ = fs::remove_file(test_file);
    let _ = fs::remove_file("test_output_processed.txt");
}

// ============================================
// 파일 스트림 읽기/쓰기 (큰 파일 처리)
// ============================================
pub fn stream_file_example() {
    use std::io::{BufRead, BufReader};

    // 큰 파일을 스트림으로 읽기
    let file_path = "large_file.txt";
    
    // 파일 생성 (예제용)
    let mut file = fs::File::create(file_path).unwrap();
    for i in 0..100 {
        writeln!(file, "Line {}", i).unwrap();
    }

    // BufReader로 한 줄씩 읽기
    match fs::File::open(file_path) {
        Ok(file) => {
            let reader = BufReader::new(file);
            println!("  파일을 한 줄씩 읽기:");
            for (i, line) in reader.lines().enumerate().take(5) {
                match line {
                    Ok(line) => println!("    {}: {}", i + 1, line),
                    Err(e) => println!("    에러: {}", e),
                }
            }
        }
        Err(e) => println!("  파일 열기 실패: {}", e),
    }

    // 정리
    let _ = fs::remove_file(file_path);
}

// ============================================
// 파일 복사 및 이동
// ============================================
pub fn file_operations_example() {
    let source = "source.txt";
    let dest = "dest.txt";

    // 소스 파일 생성
    fs::write(source, "원본 파일 내용").unwrap();

    // 파일 복사
    match fs::copy(source, dest) {
        Ok(size) => println!("  파일 복사 성공: {} bytes", size),
        Err(e) => println!("  파일 복사 실패: {}", e),
    }

    // 파일 이동 (이름 변경)
    let moved = "moved.txt";
    match fs::rename(dest, moved) {
        Ok(_) => println!("  파일 이동 성공: {} -> {}", dest, moved),
        Err(e) => println!("  파일 이동 실패: {}", e),
    }

    // 정리
    let _ = fs::remove_file(source);
    let _ = fs::remove_file(moved);
}

