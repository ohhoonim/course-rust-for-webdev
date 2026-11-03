
# Serde: Serialization/Deserialization️

## 목표

- 직렬화(Serialization) 이해: 러스트 구조체나 데이터를 JSON 등 외부 형식(바이트 또는 문자열)으로 변환하는 방법
- 역직렬화(Deserialization) 이해: 외부에서 들어온 JSON 데이터(문자열)를 러스트의 안전한 타입 구조체로 안전하게 변환하는 방법
- `serde`와 `serde_json` 활용: `#[derive(Serialize, Deserialize)]` 매크로를 사용하여 보일러플레이트 없이 데이터를 변환한다.
- 고급 속성 제어 (`#[serde(...)]`): 필드 이름 변경(예: `camelCase`와 `snake_case` 매핑), 옵션 값 처리, 기본값 설정 등 실무에서 필요한 데이터 구조 조작법을 다룬다.

## 상황: 이커머스 주문 접수 및 응답 API 데이터 매핑

클라이언트가 서버로 보낸 JSON 형태의 주문 요청 데이터를 러스트 구조체로 역직렬화하고, 주문 처리 결과 구조체를 다시 JSON 문자열로 직렬화하여 응답하는 파이프라인을 구현합니다.

- 상황 1 (주문 요청 역직렬화): 클라이언트가 전송한 JSON 문자열(`{"product_id": "P123", "quantity": 2, "coupon_code": "SUMMER_10"}`)을 받아 러스트의 `OrderRequest` 구조체로 변환합니다.
- 상황 2 (데이터 변환 및 매핑): `#[serde(rename_all = "camelCase")]` 등의 어트리뷰트를 활용해 JSON의 키 이름 스타일과 러스트 구조체의 필드 스타일을 자연스럽게 매핑합니다.
- 상황 3 (주문 결과 직렬화): 주문 성공 여부와 결제 금액이 담긴 `OrderResponse` 구조체를 생성한 뒤, 이를 다시 클라이언트에게 보낼 JSON 문자열로 직렬화합니다.

## 힌트

- dependency
    
    ```toml
    [dependencies]
    serde = { version = "1.0", features = ["derive"] }
    serde_json = "1.0"
    ```
    
- struct
    
    ```rust
    struct OrderRequest {
        product_id: String,
        quantity: u32,
        #[serde(default)]
        coupon_code: Option<String>,
    }
    
    struct OrderResponse {
        success: bool,
        order_id: String,
        total_price: u32,
        message: String,
    }
    ```
    
- drive와 매크로
    
    코드를 자동으로 생성하거나 확장해 주는 메타프로그래밍(Metaprogramming) 도구
    
    - 매크로
        
        매크로는 컴파일 타임에 코드를 생성하는 전체 기술이다.
        
        - 선언적 매크로 (`macro_rules!`)
            - 흔히 보는 `println!`이나 `vec!`처럼 패턴 매칭을 기반으로 코드를 반복 생성하는 매크로다. 정해진 규칙에 따라 코드를 치환하는 방식으로 비교적 간단하게 작성
        - 프로시저럴 매크로 (`Procedural Macro`)
            - Rust 코드를 입력(TokenStream)받아 AST(추상 구문 트리)로 분석한 뒤, 완전히 새로운 코드를 프로그래밍 방식으로 작성한다. 별도의 라이브러리 크레이트(`proc-macro = true`)로 분리해서 만들어야 합니다.
    - derive
        
        프로시저럴 매크로의 한 종류이다. 구조체나 열거형 위에 `#[derive(TraitName)]` 형태로 선언하면, 컴파일러가 해당 구조체의 이름과 필드 정보(AST)를 읽어 그에 맞는 트레이트 구현 코드를 자동으로 만들어준다.
        
        프로시저럴 매크로는 크게 세 가지 형태로 나뉘는데, `derive`는 그중 가장 많이 쓰이는 형태이다.
        
        1. 커스텀 Derive 매크로 (`#[proc_macro_derive]`)
            - `#[derive(Debug, Serialize, MyOrmModel)]`처럼 사용하며, 특정 트레이트를 자동으로 구현해 준다.
        2. 어트리뷰트 매크로 (`#[proc_macro_attribute]`)
            - 함수나 구조체 자체에 커스텀 어트리뷰트(예: 웹 프레임워크의 라우팅 설정 `#[get("/users")]`)를 붙여 코드를 통째로 변환하거나 감싼다
        3. 함수형 매크로 (`custom_macro!()`)
            - 함수 호출 형태처럼 생겨서 안의 인자를 받아 코드를 생성한다.

## 결과

- 상황 1: 역직렬화
    
    ```rust
    use serde::Deserialize;
    
    // 클라이언트가 보낸 주문 요청 JSON을 매핑할 구조체
    #[derive(Deserialize, Debug)]
    struct OrderRequest {
        product_id: String,
        quantity: u32,
        coupon_code: Option<String>, // 쿠폰은 없을 수도 있으므로 Option 처리
    }
    
    fn main() {
        // 상황 1: 클라이언트가 보낸 JSON 문자열 (가상 시나리오)
        let json_data = r#"{
            "product_id": "P12345",
            "quantity": 2,
            "coupon_code": "SUMMER_10"
        }"#;
    
        // serde_json을 이용해 JSON 문자열을 러스트 구조체로 역직렬화 (Deserialization)
        let order_result: Result<OrderRequest, serde_json::Error> = serde_json::from_str(json_data);
    
        match order_result {
            Ok(order) => {
                println!("--- [상황 1] 주문 요청 역직렬화 성공! ---");
                println!("구조체 데이터: {:?}", order);
                println!("상품 ID: {}", order.product_id);
                println!("주문 수량: {}", order.quantity);
                println!("적용 쿠폰: {:?}", order.coupon_code);
            }
            Err(e) => {
                println!("역직렬화 실패: {}", e);
            }
        }
    }
    ```
    
- 상황 2: 데이터 매핑
    
    ```rust
    use serde::Deserialize;
    
    // #[serde(rename_all = "camelCase")]를 사용하여
    // JSON의 camelCase 키들을 러스트의 snake_case 필드에 자동으로 매핑합니다.
    #[derive(Deserialize, Debug)]
    #[serde(rename_all = "camelCase")]
    struct OrderRequest {
        product_id: String,
        quantity: u32,
        
        // 특정 필드만 이름을 다르게 매핑하고 싶다면 #[serde(rename = "...")]을 쓸 수도 있습니다.
        #[serde(default)] // JSON에 값이 아예 없을 경우 기본값(Default::default() -> None)으로 처리
        coupon_code: Option<String>,
    }
    
    fn main() {
        // 상황 2: 카멜케이스(camelCase)로 전달된 JSON 데이터
        let json_data = r#"{
            "productId": "P99999",
            "quantity": 5
        }"#;
    
        let order_result: Result<OrderRequest, serde_json::Error> = serde_json::from_str(json_data);
    
        match order_result {
            Ok(order) => {
                println!("--- [상황 2] 카멜케이스 매핑 및 기본값 처리 성공! ---");
                println!("구조체 데이터: {:?}", order);
                println!("상품 ID: {}", order.product_id);
                println!("주문 수량: {}", order.quantity);
                println!("적용 쿠폰: {:?} (쿠폰이 없어 기본값 None 처리됨)", order.coupon_code);
            }
            Err(e) => {
                println!("역직렬화 실패: {}", e);
            }
        }
    }
    ```
    
- 상황 3: 직렬화
    
    ```rust
    use serde::{Deserialize, Serialize};
    
    // 1. 요청 데이터를 받는 구조체 (역직렬화)
    #[derive(Deserialize, Debug)]
    #[serde(rename_all = "camelCase")]
    struct OrderRequest {
        product_id: String,
        quantity: u32,
        #[serde(default)]
        coupon_code: Option<String>,
    }
    
    // 2. 응답 데이터를 보내는 구조체 (직렬화)
    // 클라이언트 친화적으로 응답도 camelCase로 변환되도록 설정합니다.
    #[derive(Serialize, Debug)]
    #[serde(rename_all = "camelCase")]
    struct OrderResponse {
        success: bool,
        order_id: String,
        total_price: u32,
        message: String,
    }
    
    fn main() {
        // [상황 1 & 2] 클라이언트가 보낸 JSON 요청 (역직렬화 대상)
        let json_request = r#"{
            "productId": "P12345",
            "quantity": 2,
            "coupon_code": "SUMMER_10"
        }"#;
    
        // JSON -> 러스트 구조체로 변환
        let order_req: OrderRequest = serde_json::from_str(json_request).unwrap();
        println!("--- [역직렬화 완료] 수신된 주문 요청 ---");
        println!("{:?}", order_req);
    
        // 비즈니스 로직 처리 (가상의 결제 및 총액 계산)
        let unit_price = 50_000;
        let base_price = unit_price * order_req.quantity;
        let discount = if order_req.coupon_code.as_deref() == Some("SUMMER_10") {
            base_price / 10 // 10% 할인
        } else {
            0
        };
        let total_price = base_price - discount;
    
        // [상황 3] 응답 데이터 구조체 생성
        let response_data = OrderResponse {
            success: true,
            order_id: String::from("ORD-2026-001"),
            total_price,
            message: String::from("주문이 성공적으로 완료되었습니다."),
        };
    
        // serde_json을 이용해 러스트 구조체를 JSON 문자열로 직렬화 (Serialization)
        // pretty_print(to_string_pretty)를 사용하여 들여쓰기가 예쁜 JSON 문자열로 변환합니다.
        let json_response = serde_json::to_string_pretty(&response_data).unwrap();
    
        println!("\n--- [상황 3] 주문 처리 결과 직렬화 성공 (클라이언트 응답 JSON) ---");
        println!("{}", json_response);
    }
    ```
   