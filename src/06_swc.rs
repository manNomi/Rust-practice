// ============================================
// 9. SWC Rust API 학습
// Babel AST를 SWC AST로 변환하는 방법
// ============================================

// 주의: 이 파일을 실행하려면 Cargo.toml에 swc 의존성이 필요합니다
// [dependencies]
// swc_ecma_ast = "1"
// swc_ecma_parser = "1"
// swc_ecma_visit = "1"
// swc_ecma_codegen = "1"

pub fn run_swc_examples() {
    println!("\n=== SWC Rust API 학습 ===\n");
    println!("이 예제는 SWC crate들이 필요합니다.");
    println!("Cargo.toml에 다음을 추가하세요:");
    println!("  [dependencies]");
    println!("  swc_ecma_ast = \"1\"");
    println!("  swc_ecma_parser = \"1\"");
    println!("  swc_ecma_visit = \"1\"");
    println!("  swc_ecma_codegen = \"1\"\n");
}

// 실제 사용 예제 (swc crate가 있을 때만 컴파일)
#[cfg(feature = "swc")]
pub fn swc_examples() {
    println!("=== SWC Rust API 예제 ===\n");

    // 1. 코드 파싱
    println!("1. 코드 파싱");
    parse_code_example();
    println!();

    // 2. AST Visitor 패턴
    println!("2. AST Visitor 패턴");
    visitor_pattern_example();
    println!();

    // 3. AST 수정
    println!("3. AST 수정");
    ast_modification_example();
    println!();

    // 4. 코드 생성
    println!("4. 코드 생성");
    code_generation_example();
    println!();
}

// ============================================
// TypeScript 코드 (Babel):
// traverse(ast, {
//   ImportDeclaration: (path) => {
//     // ...
//   }
// });
// ============================================

// SWC로 코드 파싱
#[cfg(feature = "swc")]
fn parse_code_example() {
    use swc_ecma_parser::{Parser, StringInput, Syntax, TsConfig};
    use swc_common::{SourceMap, FileName, Span, DUMMY_SP};

    let code = r#"
        import { t } from './translations';
        const CONSTANT = t('key');
    "#;

    let cm = SourceMap::default();
    let fm = cm.new_source_file(
        FileName::Custom("test.ts".into()),
        code.into(),
    );

    let syntax = Syntax::Typescript(TsConfig::default());
    let mut parser = Parser::new_from(
        syntax,
        StringInput::from(&*fm),
        None,
    );

    match parser.parse_module() {
        Ok(module) => {
            println!("  파싱 성공!");
            println!("  모듈 항목 수: {}", module.body.len());
        }
        Err(e) => {
            println!("  파싱 실패: {:?}", e);
        }
    }
}

// ============================================
// Visitor 패턴: AST 순회 및 처리
// TypeScript:
// traverse(ast, {
//   ImportDeclaration: (path) => { ... }
// });
// ============================================
#[cfg(feature = "swc")]
fn visitor_pattern_example() {
    use swc_ecma_ast::*;
    use swc_ecma_visit::{VisitMut, VisitMutWith};

    // Import 선언을 찾는 Visitor
    struct ImportVisitor {
        imports: Vec<String>,
    }

    impl VisitMut for ImportVisitor {
        fn visit_mut_import_decl(&mut self, import: &mut ImportDecl) {
            // Import 선언 처리
            let source = match &*import.src.value {
                s => s.to_string(),
            };
            self.imports.push(source);
        }
    }

    // 사용 예제
    let mut visitor = ImportVisitor {
        imports: Vec::new(),
    };

    // module.visit_mut_with(&mut visitor);
    println!("  Import 선언 수집 완료");
}

// ============================================
// AST 수정 예제
// ============================================
#[cfg(feature = "swc")]
fn ast_modification_example() {
    use swc_ecma_ast::*;
    use swc_ecma_visit::{VisitMut, VisitMutWith};

    // 상수 이름을 변경하는 Visitor
    struct ConstantRenamer {
        old_name: String,
        new_name: String,
    }

    impl VisitMut for ConstantRenamer {
        fn visit_mut_ident(&mut self, ident: &mut Ident) {
            if ident.sym == self.old_name.as_str() {
                ident.sym = self.new_name.as_str().into();
            }
        }
    }

    println!("  AST 수정 Visitor 예제");
}

// ============================================
// 코드 생성 예제
// ============================================
#[cfg(feature = "swc")]
fn code_generation_example() {
    use swc_ecma_codegen::{text_writer::JsWriter, Emitter};
    use swc_ecma_ast::*;
    use swc_common::{SourceMap, FileName};

    println!("  코드 생성 예제");
    // 실제 구현은 swc crate의 복잡한 API를 사용해야 함
}

// ============================================
// 실전 예제: Import 선언 수집
// TypeScript:
// traverse(ast, {
//   ImportDeclaration: (path) => {
//     const source = path.node.source.value;
//     imports.push(source);
//   }
// });
// ============================================
#[cfg(feature = "swc")]
pub fn collect_imports_example() {
    use swc_ecma_ast::*;
    use swc_ecma_visit::{Visit, VisitWith};

    struct ImportCollector {
        imports: Vec<String>,
    }

    impl Visit for ImportCollector {
        fn visit_import_decl(&mut self, import: &ImportDecl) {
            let source = import.src.value.to_string();
            self.imports.push(source);
        }
    }

    println!("  Import 수집 예제");
}

// ============================================
// 실전 예제: useTranslation 호출 찾기
// TypeScript:
// if (t.isIdentifier(path.node.callee, { name: "useTranslation" })) {
//   // ...
// }
// ============================================
#[cfg(feature = "swc")]
pub fn find_use_translation_example() {
    use swc_ecma_ast::*;
    use swc_ecma_visit::{Visit, VisitWith};

    struct UseTranslationFinder {
        found: bool,
    }

    impl Visit for UseTranslationFinder {
        fn visit_call_expr(&mut self, call: &CallExpr) {
            match &call.callee {
                Callee::Expr(expr) => {
                    if let Expr::Ident(ident) = &**expr {
                        if ident.sym == "useTranslation" {
                            self.found = true;
                        }
                    }
                }
                _ => {}
            }
        }
    }

    println!("  useTranslation 찾기 예제");
}

// ============================================
// SWC 없이 개념 설명
// ============================================
pub fn swc_concepts_explanation() {
    println!("=== SWC Rust API 개념 설명 ===\n");

    println!("1. AST 타입:");
    println!("   - Module: 전체 모듈");
    println!("   - ImportDecl: import 선언");
    println!("   - VarDecl: 변수 선언");
    println!("   - CallExpr: 함수 호출");
    println!("   - Ident: 식별자\n");

    println!("2. Visitor 패턴:");
    println!("   - Visit: 불변 참조로 순회");
    println!("   - VisitMut: 가변 참조로 순회 (수정 가능)");
    println!("   - visit_mut_with(): AST를 순회하며 visitor 실행\n");

    println!("3. 코드 생성:");
    println!("   - swc_ecma_codegen: AST를 코드로 변환");
    println!("   - Emitter: 코드 생성기\n");

    println!("4. TypeScript → Rust 변환:");
    println!("   - traverse() → visit_mut_with()");
    println!("   - path.node → 직접 AST 노드 접근");
    println!("   - t.isIdentifier() → 패턴 매칭\n");
}

