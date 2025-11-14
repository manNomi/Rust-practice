// ============================================
// 6. 비동기 프로그래밍 (async/await) 학습
// TypeScript의 async/await를 Rust로 변환하는 방법
// ============================================

// 주의: 이 파일을 실행하려면 Cargo.toml에 tokio 의존성이 필요합니다
// [dependencies]
// tokio = { version = "1", features = ["full"] }

pub fn run_async_examples() {
    println!("\n=== 비동기 프로그래밍 (async/await) 학습 ===\n");
    println!("이 예제는 tokio 런타임이 필요합니다.");
    println!("Cargo.toml에 다음을 추가하세요:");
    println!("  [dependencies]");
    println!("  tokio = {{ version = \"1\", features = [\"full\"] }}\n");
}

// ============================================
// TypeScript 코드:
// public async processFiles(): Promise<{ processedFiles: string[] }> {
//   const filePaths = await glob(this.config.sourcePattern);
//   // ...
// }
// ============================================

// 비동기 함수 선언
#[tokio::main]
pub async fn async_main_example() {
    println!("=== 비동기 프로그래밍 예제 ===\n");

    // 1. 기본 async/await
    println!("1. 기본 async/await");
    basic_async_await().await;
    println!();

    // 2. 여러 비동기 작업 병렬 실행
    println!("2. 병렬 실행 (join!)");
    parallel_execution().await;
    println!();

    // 3. 비동기 파일 읽기
    println!("3. 비동기 파일 읽기");
    async_file_reading().await;
    println!();

    // 4. 채널 (Channel)
    println!("4. 채널 (Channel)");
    channel_example().await;
    println!();

    // 5. 실전 예제: processFiles
    println!("5. 실전 예제: processFiles");
    process_files_example().await;
    println!();
}

// ============================================
// 기본 async/await
// ============================================
async fn basic_async_await() {
    // async 함수는 Future를 반환
    async fn fetch_data() -> String {
        // 시뮬레이션: 네트워크 요청
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
        "데이터".to_string()
    }

    // await로 결과 기다리기
    let result = fetch_data().await;
    println!("  결과: {}", result);

    // 여러 비동기 함수 호출
    let data1 = fetch_data().await;
    let data2 = fetch_data().await;
    println!("  data1: {}, data2: {}", data1, data2);
}

// ============================================
// 병렬 실행: join!, try_join!
// ============================================
async fn parallel_execution() {
    async fn task1() -> i32 {
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
        10
    }

    async fn task2() -> i32 {
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
        20
    }

    // 순차 실행 (느림)
    let start = std::time::Instant::now();
    let result1 = task1().await;
    let result2 = task2().await;
    let sequential_time = start.elapsed();
    println!("  순차 실행: {}ms, 결과: {}, {}", 
             sequential_time.as_millis(), result1, result2);

    // 병렬 실행 (빠름)
    let start = std::time::Instant::now();
    let (result1, result2) = tokio::join!(task1(), task2());
    let parallel_time = start.elapsed();
    println!("  병렬 실행: {}ms, 결과: {}, {}", 
             parallel_time.as_millis(), result1, result2);

    // Result를 반환하는 경우: try_join!
    async fn task_with_result() -> Result<i32, String> {
        Ok(42)
    }

    match tokio::try_join!(task_with_result(), task_with_result()) {
        Ok((a, b)) => println!("  try_join 성공: {}, {}", a, b),
        Err(e) => println!("  try_join 실패: {}", e),
    }
}

// ============================================
// 비동기 파일 읽기
// ============================================
async fn async_file_reading() {
    use tokio::fs;

    // 비동기 파일 읽기
    match fs::read_to_string("test.txt").await {
        Ok(content) => println!("  파일 내용: {} bytes", content.len()),
        Err(e) => println!("  파일 읽기 실패: {}", e),
    }

    // 여러 파일 병렬 읽기
    let file1 = fs::read_to_string("file1.txt");
    let file2 = fs::read_to_string("file2.txt");
    
    match tokio::try_join!(file1, file2) {
        Ok((content1, content2)) => {
            println!("  file1: {} bytes", content1.len());
            println!("  file2: {} bytes", content2.len());
        }
        Err(e) => println!("  파일 읽기 실패: {}", e),
    }
}

// ============================================
// 채널 (Channel): 비동기 통신
// ============================================
async fn channel_example() {
    use tokio::sync::mpsc;

    // 송신자(sender)와 수신자(receiver) 생성
    let (tx, mut rx) = mpsc::channel(32);

    // 비동기 태스크에서 메시지 전송
    tokio::spawn(async move {
        for i in 0..5 {
            tx.send(i).await.unwrap();
            tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
        }
    });

    // 메시지 수신
    while let Some(message) = rx.recv().await {
        println!("  수신: {}", message);
    }
}

// ============================================
// 실전 예제: processFiles
// TypeScript:
// public async processFiles(): Promise<{ processedFiles: string[] }> {
//   const filePaths = await glob(this.config.sourcePattern);
//   const processedFiles: string[] = [];
//   for (const filePath of filePaths) {
//     const result = await processFile(filePath);
//     processedFiles.push(result);
//   }
//   return { processedFiles };
// }
// ============================================
async fn process_files_example() {
    use tokio::fs;
    use std::path::Path;

    // 파일 경로 목록 (실제로는 glob 사용)
    let file_paths = vec!["file1.txt", "file2.txt", "file3.txt"];

    // 순차 처리
    let mut processed_files = Vec::new();
    for file_path in &file_paths {
        if let Ok(content) = fs::read_to_string(file_path).await {
            processed_files.push(format!("처리됨: {}", file_path));
        }
    }
    println!("  순차 처리 결과: {:?}", processed_files);

    // 병렬 처리 (더 빠름)
    let mut handles = Vec::new();
    for file_path in file_paths {
        let handle = tokio::spawn(async move {
            match fs::read_to_string(file_path).await {
                Ok(_) => Some(format!("처리됨: {}", file_path)),
                Err(_) => None,
            }
        });
        handles.push(handle);
    }

    let mut results = Vec::new();
    for handle in handles {
        if let Ok(Some(result)) = handle.await {
            results.push(result);
        }
    }
    println!("  병렬 처리 결과: {:?}", results);
}

// ============================================
// select!: 여러 Future 중 먼저 완료된 것 선택
// ============================================
pub async fn select_example() {
    use tokio::time::{sleep, Duration};

    tokio::select! {
        result = async_task_1() => {
            println!("  task1 완료: {}", result);
        }
        result = async_task_2() => {
            println!("  task2 완료: {}", result);
        }
    }
}

async fn async_task_1() -> i32 {
    sleep(Duration::from_millis(200)).await;
    1
}

async fn async_task_2() -> i32 {
    sleep(Duration::from_millis(100)).await;
    2
}

// ============================================
// spawn: 백그라운드 태스크 실행
// ============================================
pub async fn spawn_example() {
    let handle = tokio::spawn(async {
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
        "완료".to_string()
    });

    // 다른 작업 수행
    println!("  다른 작업 수행 중...");

    // 결과 기다리기
    match handle.await {
        Ok(result) => println!("  결과: {}", result),
        Err(e) => println!("  에러: {:?}", e),
    }
}

