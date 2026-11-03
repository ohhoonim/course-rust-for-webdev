# Cargo 빌드 도구로 프로젝트 생성하기

## Cargo로 새 프로젝트 만들기

```bash
$ cargo new [프로젝트명]
```

- `[프로젝트명]` 부분은 폴더명이 된다.
- 명령어를실행하면  `[프로젝트명]` 으로된 폴더가 생성된다.

## VSCode에서 프로젝트 열기

```bash
$ cd [프로젝트명]
$ code .
```

## 프로젝트 주요 파일 해설

- main.rs
    
    ```rust
    fn main() {
        println!("Hello, world!");
    }
    ```
    
    - `println!` 러스트에서 느낌표(!) 가 붙어있는 것을 ‘매크로’라고 한다.
    - `fn main()` 함수: 프로그램이 실행되는 시작점
    
    <aside>
    💡
    
    매크로
    
    ‘코드를 만들어주는 코드’ 정도로 이해하자. `println!` 매크로를 사용하지 않을 경우 다음과 같은 추가 코딩이 필요하다.
    
    ```rust
    use std::io::{self, Write};
    
    fn main() -> io::Result<()> {
        // 1. 표준 출력(stdout) 핸들을 가져옵니다.
        let mut stdout = io::stdout();
    
        // 2. 문자열을 바이트 슬라이스(b"...") 형태로 변환하여 직접 씁니다.
        // 매크로와 달리 줄바꿈 문자(\n)를 직접 넣어주어야 합니다.
        stdout.write_all(b"Hello, world!\n")?;
    
        // 3. 버퍼를 비워 화면에 즉시 출력되도록 합니다.
        stdout.flush()?;
    
        Ok(())
    }
    ```
    
    </aside>
    
- Cargo.toml
    
    ```toml
    [package]
    name = "course-rust-for-webdev"
    version = "0.1.0"
    edition = "2024"
    
    [dependencies]
    
    ```
    
    - `[package]`: 프로젝트 메타데이터를 정의. 패키지명, 버전, 러스트 에디션
    - `[dependencies]`: 외부 크레이트(라이브러기)를 지정
    - 그 외 빌드 프로파일 설정과 워크스페이스를 관리할 수 있다.
    - (참고) Cargo.lock 파일은 실제 다운로드된 의존성의 정확한 버전을 고정하고 기록한다.
    
    <aside>
    💡
    
    크레이트
    
    rust에서는 ‘라이브러리’라는 용어 대신 ‘크레이트’라는 용어를 사용한다. 
    
    </aside>
    

## 프로그램 실행하기

- IDE에서 실행하기
    
    main.rs 파일의 `fn main()`  위에 ‘▶ Run’ 부분을 클릭한다.
    
- 터미널에서 실행하기
    
    ```bash
    $ cargo run 
    ```
    
- 실행이 안될때 윈도우에서 추가 조치방법
    
    Visual Studio가 설치되어 있지 않은 경우(Visual Studio Code 아님), rustup-init이 사전 요구 사항을 자동으로 설치하지만 모든 사용자에게 적합하지 않을 수 있는 Visual Studio Community 에디션을 설치한다. 개인, 학계 및 오픈소스 사용에는 무료이지만, 회사내 조직에 속한 경우 어떤 에디션이 자신에게 적합한지 확인해야한다.
    
    https://rust-lang.github.io/rustup/installation/windows-msvc.html 을 참고하십시오. 
    

## (참고) Cargo의 주요 표준 타겟 디렉토리


Cargo의 주요 표준 타겟 디렉토리

Cargo는 코드의 목적에 따라 체계적으로 분리하여 사용하도록 표준구조를 지원한다.

Cargo에 대한 자세한 내용은 https://doc.rust-lang.org/cargo/index.html 에서 찾아볼 수 있다.

- `src/main.rs`: 메인 실행 프로그램 (cargo run)
- `src/lib.rs`: 라이브러리 크레이트. 다른 프로그램에서 사용할 수 있도록 모듈이나 함수를 묶어준다.
- `src/bin/`: 보조 실행 파일. main() 함수를 사용할 수 있다. (cargo run —bin 파일이름)
- `examples/`: 사용법 예제 코드들. main() 함수를 사용할 수 있다.( cargo run —example 파일이름)
- `tests/`: 이 폴더의 소스들은 통합테스트 파일로 인식한다. (cargo test)
- `benches/`: 벤치마크 코드를 작성하는 폴더이다. (cargo bench)

