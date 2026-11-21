# Modules, Cates and Project Organization ️

## 목표

모듈, 크레이트, 워크스페이스를 이해하고 프로젝트를 적절히 구성한다.

## 강의 영상

- 유튜브: [웹 프로그래밍 개발자를 위한 Rust - 2부 활용하기 (Modules, Cates and Project Organization ️)](https://youtu.be/k1_wcfpJZWo)

## 상황: 모든 코드를 main() 함수에 넣을 수는 없다. 프로젝트내 파일들을 목적에 맞게 분리한다.

1. ‘tactics-rust-warehouse’ 명으로 프로젝트를 생성하고 main.rc에 ‘Product’ stuct를 작성하기
2. struct 를 ‘model.rs’ 파일로 분리하기(모듈화) 
3. workspace를 사용하여  크래이트를 ‘core’와 ‘cli’ 로 나누기
    1. core: 라이브러리
    2. cli: 바이너리, core를 dependency 한다.
4. model.rs는 core로 이동시키고, main.rs의 소스를 cli로 옮기기

## 힌트

- 네이밍 컨벤션: 프로젝트명(패키지명)은 하이픈(-)을 사용하고 변수명은 언더스코어(_)를 사용한다. 코드에서 use 사용시 하이픈을 언더스코어로 rust가 자동 변환해준다.
- 프로젝트 생성
    
    ```bash
    $ cargo new tactics-rust-warehouse
    $ cd tactics-rust-warehouse
    $ code .
    ```
    
- dependences
    
    ```bash
    $ cargo add chrono serde --features serde/derive uuid --features uuid/v4
    
    $ cargo add uuid --features serde,v4
    $ cargo add chrono --features serde
    ```
    
    또는 Cargo.toml 을 직접수정
    
    ```toml
    [dependencies]
    chrono = { version = "0.4.45", features = ["serde"] }
    serde = { version = "1.0.229", features = ["derive"] }
    uuid = { version = "1.28.0", features = ["serde", "v4"] }
    ```
    
- structs
    
    ```rust
    
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Dimensions {
        pub length: f64,
        pub width: f64,
        pub height: f64,
    }
    
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Product {
        pub id: Uuid,                  // 상품 고유 식별자 (UUID)
        pub sku: String,               // 재고 관리 코드 (Stock Keeping Unit)
        pub name: String,              // 상품명
        pub description: Option<String>, // 상품 상세 설명
        pub category: String,          // 상품 카테고리
        pub unit_price: u32,           // 단가 (원화 기준 정수형 권장)
        pub weight: f64,               // 중량 (kg 단위)
        pub dimensions: Dimensions,    // 치수 (가로, 세로, 높이)
        pub stock_quantity: u32,       // 현재 창고 재고 수량
        pub reorder_level: u32,        // 안전재고 기준선 (부족 시 발주 알림)
        pub location: Option<String>,  // 창고 내 보관 위치 (예: Zone-A, Rack-03)
        pub created_at: DateTime<Utc>, // 등록 일시
        pub updated_at: DateTime<Utc>, // 최종 수정 일시
    }
    ```
    
- workspace 로 변경한 뒤 Cargo.toml
    
    ```toml
    ############################# root cargo
    [workspace]
    members = ["core", "cli"]
    
    [workspace.dependencies]
    chrono = { version = "0.4.45", features = ["serde"] }
    serde = { version = "1.0.229", features = ["derive"] }
    uuid = { version = "1.28.0", features = ["serde", "v4"] }
    
    ############################ core
    [package]
    name = "core"
    version = "0.1.0"
    edition = "2024"
    
    [dependencies]
    serde.workspace = true
    uuid.workspace = true
    chrono.workspace = true
    
    ########################## cli
    [package]
    name = "cli"
    version = "0.1.0"
    edition = "2024"
    
    [dependencies]
    core = { path = "../core" }
    uuid.workspace = true
    chrono.workspace= true
        
    ```
    

## 결과

(생략)

## 과제

- Crate에서 커스텀 feature를 제공하는 방법을 조사하시오.
- cargo build 시 crate 단위로 빌드하려면 어떤 옵션을 사용해야하는가?