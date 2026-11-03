# Rust 프로그래밍 과정 소개

## 대상
Rust 프로그래밍 과정은 Rust를 처음 접하시는 분들 보다는 프로그래밍 언어를 한번이라도 접했봤다고 가정합니다. 기본 제어흐름, 변수 타입 등에 대한 기초지식을 요구합니다. 시스템 프로그래밍보다는 웹 프로그래밍에 초점을 맞추었습니니다.

## 활용 방법

- 1부 시작하기: 필수 문법과 Rust만의 특징을 이해. 👉 [실습코드](../../a01-rust-beginner/README.md)
- 2부 프로젝트 실전: 프로젝트를 구성하고 실전 코딩시 필요한 기법 연습. 👉 [실습코드](../../a02-rust-intermediate/README.md)

## 이 문서를 로컬에서 실행하는 법

```sh
$ cd z01-rust-grammar

# mdbook이 이미 설치되어있다면 skip
$ cargo install mdbook

# 3001포트로 로컬 실행. http://localhost:3001
$ mdbook serve --port 3001
```