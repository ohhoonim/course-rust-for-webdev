# 개발환경 구성

윈도우용 프로그램을 개발하는 것이 아니라면, 윈도우의 WSL, Linux, Macos에서 개발하는 것을 권장합니다.

## Rust 설치

- rustup.rs 사이트 방문
    - 페이지에 있는 명령어를 카피하여 터미널에서 실행
- rustup을 사용하면 rustc와 cargo를 설치해준다.
    - rustc: 컴파일러
    - cargo: rust 빌드 도구

## 설치확인

```bash
$ rustc --version
$ cargo --version
```

## rustup 업데이트

```bash
$ rustup update
```

## IDE

처음하시는 분이라면 VSCode로 시작하는 것을 권장합니다.

- RustRover: jetbrains 에서 만든 Rust 전용 IDE
    - 다운로드: https://www.jetbrains.com/rust/
- Visual Studio Code: Microsoft에서 만든 범용 IDE
    - 다운로드: https://code.visualstudio.com/docs/languages/rust
    - rust-analyzer 확장 프로그램을 추가해준다.