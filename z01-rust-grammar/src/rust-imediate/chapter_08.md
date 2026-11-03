# File I/O &amp; Working with Paths

## 목표

- 표준 파일 I/O 기본기 숙달: 표준 라이브러리(`std::fs`, `std::io`)를 활용하여 파일을 생성, 읽기, 쓰기, 수정하는 방법
- 에러 핸들링: 파일이 존재하지 않거나 권한이 없는 등 파일 I/O 과정에서 발생하는 예외 상황을 `Result`와 `?` 연산자로 안전하게 처리합니다.
- 버퍼드 I/O(Buffered I/O) 활용: 성능 최적화를 위해 버퍼를 사용하는 `BufReader`와 `BufWriter`의 개념과 사용법을 이해

## 상황: 이커머스 시스템 설정 로드 및 주문 감사 로그(Audit Log) 기록

- 상황 1 (설정 파일 읽기): 서버가 구동될 때 로컬 경로에 있는 설정 파일(config/settings.json)을 읽어와 러스트 구조체로 역직렬화하여 시스템 환경 변수를 세팅한다.
- 상황 2 (주문 감사 로그 기록): 사용자가 결제를 완료할 때마다 상세 주문 내역을 텍스트 로그 파일(`order_audit.log`)에 누락 없이 이어쓰기(Append) 방식으로 기록한다.

## 힌트

- dependency
    
    ```toml
    [dependencies]
    serde = { version = "1.0", features = ["derive"] }
    serde_json = "1.0"
    ```
    
- struct
    
    ```rust
    // [상황 1] 설정 파일 구조체
    #[derive(Deserialize, Serialize, Debug)]
    #[serde(rename_all = "camelCase")]
    struct AppConfig {
        server_host: String,
        server_port: u16,
        max_connections: usize,
    }
    
    // [상황 2] 감사 로그 구조체
    #[derive(Serialize, Debug)]
    struct AuditLog {
        timestamp: String,
        order_id: String,
        action: String,
        amount: u32,
    }
    ```
    
- `?` 에러 전파 연산자
    
    러스트에서 `?` 연산자는 '에러 전파 연산자(Error Propagation Operator)'이다.
    
    에러가 발생할 수 있는 연산 뒤에 붙여서, 코드를 매우 간결하게 작성할 수 있도록 한다.
    
    1. 성공(`Ok`)한 경우: `Ok(값)` 안에 들어 있는 실제 데이터를 안전하게 꺼내어(`unwrap`) 변수에 담고 다음 코드를 계속 실행
    2. 에러(`Err`)인 경우: 남은 코드를 더 이상 실행하지 않고 즉시 현재 함수를 빠져나와, 해당 에러를 호출자(부모 함수)에게 곧바로 반환(Early Return)
    
    이 `?` 연산자를 사용하려면 현재 함수가 결과를 반환하는 `Result` 타입이어야 한다
    

## 결과

```rust
use serde::{Deserialize, Serialize};
use std::fs::{self, OpenOptions};
use std::io::{self, BufReader, BufWriter, Write};
use std::path::Path;

// [상황 1] 설정 파일 구조체
#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
struct AppConfig {
    server_host: String,
    server_port: u16,
    max_connections: usize,
}

// [상황 2] 감사 로그 구조체
#[derive(Serialize, Debug)]
struct AuditLog {
    timestamp: String,
    order_id: String,
    action: String,
    amount: u32,
}

fn main() -> io::Result<()> {
    let config_path = "config.json";
    let log_path = "order_audit.log";

    // -------------------------------------------------------------
    // [상황 1] 설정 파일 읽기 (없을 경우 기본 파일을 먼저 생성해 둠)
    // -------------------------------------------------------------
    if !Path::new(config_path).exists() {
        let default_config = AppConfig {
            server_host: String::from("127.0.0.1"),
            server_port: 8080,
            max_connections: 1000,
        };
        let json_str = serde_json::to_string_pretty(&default_config)?;
        fs::write(config_path, json_str)?;
        println!("초기 설정 파일('{}')이 생성되었습니다.", config_path);
    }

    // 파일 열기 및 BufReader를 이용한 고성능 읽기 & 역직렬화
    let config_file = fs::File::open(config_path)?;
    let reader = BufReader::new(config_file);
    let config: AppConfig = serde_json::from_reader(reader)?;

    println!("--- [상황 1] 설정 파일 로드 성공 ---");
    println!("서버 주소: {}:{}", config.server_host, config.server_port);
    println!("최대 동시 접속 수: {}\n", config.max_connections);

    // -------------------------------------------------------------
    // [상황 2] 주문 감사 로그(Audit Log) 이어쓰기 (Append)
    // -------------------------------------------------------------
    // OpenOptions를 사용하여 파일이 없으면 생성하고, 있으면 끝에 이어서 작성(append(true))합니다.
    let log_file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(log_path)?;

    let mut writer = BufWriter::new(log_file);

    // 가상의 주문 완료 로그 데이터 생성
    let new_log = AuditLog {
        timestamp: String::from("2026-03-31T12:34:56Z"),
        order_id: String::from("ORD-2026-001"),
        action: String::from("CHECKOUT_SUCCESS"),
        amount: 1_350_000,
    };

    // 구조체를 JSON 한 줄(Line-delimited JSON)로 변환하여 로그 파일에 기록
    let log_json = serde_json::to_string(&new_log)?;
    writeln!(writer, "{}", log_json)?;
    writer.flush()?; // 버퍼에 남은 데이터를 파일에 물리적으로 반영

    println!("--- [상황 2] 주문 감사 로그 기록 성공 ---");
    println!("기록된 로그 파일 경로: {}", log_path);
    println!("로그 내용: {}", log_json);

    Ok(())
}
```