// ============================================
// 10. CLI 도구 개발 학습
// TypeScript의 commander를 Rust clap으로 변환하는 방법
// ============================================

// 주의: 이 파일을 실행하려면 Cargo.toml에 clap 의존성이 필요합니다
// [dependencies]
// clap = { version = "4", features = ["derive"] }

pub fn run_cli_examples() {
    println!("\n=== CLI 도구 개발 학습 ===\n");
    println!("이 예제는 clap crate가 필요합니다.");
    println!("Cargo.toml에 다음을 추가하세요:");
    println!("  [dependencies]");
    println!("  clap = {{ version = \"4\", features = [\"derive\"] }}\n");
}

// 실제 사용 예제 (clap crate가 있을 때만 컴파일)
#[cfg(feature = "clap")]
pub fn cli_examples() {
    use clap::Parser;

    // 명령줄 인자 파싱
    let args = CliArgs::parse();
    
    println!("=== CLI 도구 예제 ===\n");
    println!("dry-run: {}", args.dry_run);
    if let Some(pattern) = &args.pattern {
        println!("pattern: {}", pattern);
    }
    if let Some(output) = &args.output {
        println!("output: {}", output);
    }
}

// ============================================
// TypeScript 코드:
// import { Command } from "commander";
// const program = new Command();
// program
//   .option("-d, --dry-run", "Dry run mode")
//   .option("-p, --pattern <pattern>", "Source pattern")
//   .parse();
// ============================================

#[cfg(feature = "clap")]
#[derive(Parser, Debug)]
#[command(name = "i18n-wrapper")]
#[command(about = "i18n wrapper tool", long_about = None)]
struct CliArgs {
    /// Dry run mode (실제로 파일을 수정하지 않음)
    #[arg(short, long)]
    dry_run: bool,

    /// Source pattern (예: "src/**/*.ts")
    #[arg(short, long)]
    pattern: Option<String>,

    /// Output directory
    #[arg(short, long)]
    output: Option<String>,

    /// Verbose output
    #[arg(short, long)]
    verbose: bool,
}

// ============================================
// 간단한 CLI 예제 (clap 없이)
// ============================================
pub fn simple_cli_example() {
    use std::env;

    println!("=== 간단한 CLI 예제 (std::env 사용) ===\n");

    // 명령줄 인자 가져오기
    let args: Vec<String> = env::args().collect();

    println!("프로그램 이름: {:?}", args.get(0));
    println!("인자들: {:?}", &args[1..]);

    // 환경 변수 읽기
    if let Ok(value) = env::var("HOME") {
        println!("HOME 환경 변수: {}", value);
    }

    // 특정 인자 찾기
    let dry_run = args.iter().any(|arg| arg == "--dry-run" || arg == "-d");
    println!("dry-run 모드: {}", dry_run);

    // 인자 파싱
    for (i, arg) in args.iter().enumerate() {
        if arg == "--pattern" || arg == "-p" {
            if let Some(pattern) = args.get(i + 1) {
                println!("패턴: {}", pattern);
            }
        }
    }
}

// ============================================
// 로깅 예제
// ============================================
pub fn logging_example() {
    println!("=== 로깅 예제 ===\n");

    // 간단한 로깅 (println! 사용)
    println!("  [INFO] 정보 메시지");
    eprintln!("  [ERROR] 에러 메시지");

    // env_logger 사용 예제 (의존성 필요)
    // env_logger::init();
    // log::info!("정보 메시지");
    // log::error!("에러 메시지");
    // log::warn!("경고 메시지");
    // log::debug!("디버그 메시지");
}

// ============================================
// 실전 예제: i18n-wrapper CLI
// ============================================
#[cfg(feature = "clap")]
pub fn i18n_wrapper_cli() {
    use clap::{Parser, Subcommand};

    #[derive(Parser)]
    #[command(name = "i18n-wrapper")]
    #[command(about = "i18n translation wrapper tool")]
    struct Cli {
        #[command(subcommand)]
        command: Option<Commands>,

        /// Dry run mode
        #[arg(short, long, global = true)]
        dry_run: bool,

        /// Verbose output
        #[arg(short, long, global = true)]
        verbose: bool,
    }

    #[derive(Subcommand)]
    enum Commands {
        /// Process files
        Process {
            /// Source pattern
            #[arg(short, long)]
            pattern: Option<String>,
        },
        /// Analyze files
        Analyze {
            /// File path
            #[arg(short, long)]
            file: String,
        },
    }

    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Process { pattern }) => {
            println!("Processing files...");
            if let Some(p) = pattern {
                println!("Pattern: {}", p);
            }
            if cli.dry_run {
                println!("(Dry run mode)");
            }
        }
        Some(Commands::Analyze { file }) => {
            println!("Analyzing file: {}", file);
        }
        None => {
            println!("No command specified");
        }
    }
}

// ============================================
// 프로세스 종료 코드
// ============================================
pub fn exit_codes_example() {
    use std::process;

    println!("=== 프로세스 종료 코드 예제 ===\n");

    // 성공 종료
    // process::exit(0);

    // 에러 종료
    // process::exit(1);

    println!("  성공: exit code 0");
    println!("  에러: exit code 1");
    println!("  (실제 종료는 하지 않음)");
}

// ============================================
// 입력 읽기 예제
// ============================================
pub fn input_reading_example() {
    use std::io::{self, Write};

    println!("=== 입력 읽기 예제 ===\n");

    print!("  이름을 입력하세요: ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            let name = input.trim();
            println!("  안녕하세요, {}님!", name);
        }
        Err(e) => {
            eprintln!("  입력 읽기 실패: {}", e);
        }
    }
}

