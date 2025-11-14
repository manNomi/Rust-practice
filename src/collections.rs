// ============================================
// 4. 컬렉션 (HashMap, HashSet) 학습 예제
// TypeScript의 Map, Set을 Rust로 변환하는 방법
// ============================================

use std::collections::{HashMap, HashSet};

pub fn run_collections_examples() {
    println!("\n=== 컬렉션 (HashMap, HashSet) 학습 ===\n");

    // ============================================
    // 1. HashMap (TypeScript의 Map 대체)
    // ============================================
    println!("1. HashMap 예제");
    hashmap_examples();
    println!();

    // ============================================
    // 2. HashSet (TypeScript의 Set 대체)
    // ============================================
    println!("2. HashSet 예제");
    hashset_examples();
    println!();

    // ============================================
    // 3. 중첩 컬렉션 (Map<string, Set<string>>)
    // ============================================
    println!("3. 중첩 컬렉션 예제");
    nested_collections_example();
    println!();

    // ============================================
    // 4. Entry API (효율적인 삽입/업데이트)
    // ============================================
    println!("4. Entry API 예제");
    entry_api_example();
    println!();

    // ============================================
    // 5. 이터레이터 활용
    // ============================================
    println!("5. 이터레이터 예제");
    iterator_examples();
    println!();
}

// ============================================
// TypeScript 코드:
// const map = new Map<string, string>();
// map.set("key", "value");
// ============================================
fn hashmap_examples() {
    // HashMap 생성
    let mut map: HashMap<String, String> = HashMap::new();

    // 값 삽입
    map.insert("name".to_string(), "Alice".to_string());
    map.insert("age".to_string(), "30".to_string());
    map.insert("city".to_string(), "Seoul".to_string());

    println!("  HashMap: {:?}", map);

    // 값 가져오기
    if let Some(name) = map.get("name") {
        println!("  name: {}", name);
    }

    // 값이 없으면 기본값 반환
    let country = map.get("country").unwrap_or(&"Unknown".to_string());
    println!("  country: {}", country);

    // 값 존재 확인
    if map.contains_key("age") {
        println!("  'age' 키가 존재합니다");
    }

    // 값 제거
    let removed = map.remove("city");
    println!("  제거된 값: {:?}", removed);
    println!("  제거 후: {:?}", map);

    // 모든 키-값 순회
    println!("  모든 항목:");
    for (key, value) in &map {
        println!("    {}: {}", key, value);
    }
}

// ============================================
// TypeScript 코드:
// const set = new Set<string>();
// set.add("value");
// ============================================
fn hashset_examples() {
    // HashSet 생성
    let mut set: HashSet<String> = HashSet::new();

    // 값 추가
    set.insert("apple".to_string());
    set.insert("banana".to_string());
    set.insert("orange".to_string());
    set.insert("apple".to_string()); // 중복은 무시됨

    println!("  HashSet: {:?}", set);
    println!("  크기: {}", set.len());

    // 값 존재 확인
    if set.contains("apple") {
        println!("  'apple'이 존재합니다");
    }

    // 값 제거
    set.remove("banana");
    println!("  'banana' 제거 후: {:?}", set);

    // 집합 연산
    let set1: HashSet<i32> = [1, 2, 3, 4].iter().cloned().collect();
    let set2: HashSet<i32> = [3, 4, 5, 6].iter().cloned().collect();

    println!("  set1: {:?}", set1);
    println!("  set2: {:?}", set2);

    // 교집합
    let intersection: HashSet<_> = set1.intersection(&set2).cloned().collect();
    println!("  교집합: {:?}", intersection);

    // 합집합
    let union: HashSet<_> = set1.union(&set2).cloned().collect();
    println!("  합집합: {:?}", union);

    // 차집합
    let difference: HashSet<_> = set1.difference(&set2).cloned().collect();
    println!("  차집합: {:?}", difference);
}

// ============================================
// TypeScript 코드:
// const map = new Map<string, Set<string>>();
// const set = new Set(["value1", "value2"]);
// map.set("key", set);
// ============================================
fn nested_collections_example() {
    // TypeScript의 Map<string, Set<string>>를 Rust로 변환
    let mut constants_with_renderable_props: HashMap<String, HashSet<String>> = HashMap::new();

    // 값 추가
    let mut props1 = HashSet::new();
    props1.insert("title".to_string());
    props1.insert("description".to_string());
    constants_with_renderable_props.insert("CONSTANT_1".to_string(), props1);

    let mut props2 = HashSet::new();
    props2.insert("label".to_string());
    props2.insert("placeholder".to_string());
    constants_with_renderable_props.insert("CONSTANT_2".to_string(), props2);

    println!("  중첩 컬렉션:");
    for (key, props) in &constants_with_renderable_props {
        println!("    {}: {:?}", key, props);
    }

    // 특정 키의 Set에 값 추가
    if let Some(props) = constants_with_renderable_props.get_mut("CONSTANT_1") {
        props.insert("new_prop".to_string());
        println!("  CONSTANT_1에 'new_prop' 추가 후: {:?}", props);
    }

    // Entry API로 안전하게 추가
    constants_with_renderable_props
        .entry("CONSTANT_3".to_string())
        .or_insert_with(HashSet::new)
        .insert("value1".to_string());
    
    println!("  Entry API로 추가 후: {:?}", constants_with_renderable_props);
}

// ============================================
// Entry API: 효율적인 삽입/업데이트
// ============================================
fn entry_api_example() {
    let mut map: HashMap<String, i32> = HashMap::new();

    // 기존 값이 있으면 증가, 없으면 1로 초기화
    let text = "hello world hello rust world";
    for word in text.split_whitespace() {
        *map.entry(word.to_string()).or_insert(0) += 1;
    }

    println!("  단어 빈도:");
    for (word, count) in &map {
        println!("    {}: {}", word, count);
    }

    // or_insert_with: 클로저로 초기값 생성
    let mut config: HashMap<String, Vec<String>> = HashMap::new();
    config
        .entry("features".to_string())
        .or_insert_with(|| vec!["default".to_string()])
        .push("async".to_string());

    println!("  config: {:?}", config);
}

// ============================================
// 이터레이터 활용
// ============================================
fn iterator_examples() {
    let mut map: HashMap<String, i32> = HashMap::new();
    map.insert("a".to_string(), 1);
    map.insert("b".to_string(), 2);
    map.insert("c".to_string(), 3);

    // iter(): 불변 참조로 순회
    println!("  iter() - 불변 참조:");
    for (key, value) in map.iter() {
        println!("    {}: {}", key, value);
    }

    // iter_mut(): 가변 참조로 순회
    let mut map2: HashMap<String, i32> = HashMap::new();
    map2.insert("x".to_string(), 10);
    map2.insert("y".to_string(), 20);
    
    for (_, value) in map2.iter_mut() {
        *value *= 2; // 값 수정
    }
    println!("  iter_mut() 후: {:?}", map2);

    // map, filter, collect
    let numbers = vec![1, 2, 3, 4, 5];
    let squared: Vec<i32> = numbers.iter().map(|x| x * x).collect();
    println!("  제곱: {:?}", squared);

    let evens: Vec<i32> = numbers.iter().filter(|x| *x % 2 == 0).cloned().collect();
    println!("  짝수: {:?}", evens);

    // HashMap에서 값만 추출
    let values: Vec<i32> = map2.values().cloned().collect();
    println!("  값들: {:?}", values);

    // HashSet에서 필터링
    let set: HashSet<i32> = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10].iter().cloned().collect();
    let large_numbers: HashSet<i32> = set.iter().filter(|x| **x > 5).cloned().collect();
    println!("  5보다 큰 수: {:?}", large_numbers);
}

// ============================================
// 실전 예제: TranslationWrapper 스타일
// ============================================
pub struct TranslationWrapper {
    constants_with_renderable_props: HashMap<String, HashSet<String>>,
    imported_constants: HashMap<String, String>,
    analyzed_external_files: HashSet<String>,
}

impl TranslationWrapper {
    pub fn new() -> Self {
        Self {
            constants_with_renderable_props: HashMap::new(),
            imported_constants: HashMap::new(),
            analyzed_external_files: HashSet::new(),
        }
    }

    pub fn add_constant(&mut self, name: String, props: HashSet<String>) {
        self.constants_with_renderable_props.insert(name, props);
    }

    pub fn import_constant(&mut self, alias: String, path: String) {
        self.imported_constants.insert(alias, path);
    }

    pub fn mark_file_analyzed(&mut self, file_path: String) {
        self.analyzed_external_files.insert(file_path);
    }

    pub fn is_file_analyzed(&self, file_path: &str) -> bool {
        self.analyzed_external_files.contains(file_path)
    }

    pub fn get_imported_path(&self, alias: &str) -> Option<&String> {
        self.imported_constants.get(alias)
    }
}

pub fn translation_wrapper_example() {
    println!("실전 예제: TranslationWrapper");
    
    let mut wrapper = TranslationWrapper::new();
    
    // 상수 추가
    let mut props = HashSet::new();
    props.insert("title".to_string());
    props.insert("description".to_string());
    wrapper.add_constant("CONSTANT_1".to_string(), props);
    
    // 임포트 추가
    wrapper.import_constant("t".to_string(), "./translations".to_string());
    
    // 파일 분석 표시
    wrapper.mark_file_analyzed("./src/main.ts".to_string());
    
    println!("  파일 분석됨: {}", wrapper.is_file_analyzed("./src/main.ts"));
    println!("  임포트 경로: {:?}", wrapper.get_imported_path("t"));
}

