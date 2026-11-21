# Project 생성하기

## trunk 설치

```bash
$ cargo install --locked trunk
$ rustup target add wasm32-unknown-unknown
```

## dependencies

```bash
$ cargo add leptos --features=csr
$ cargo add console_error_panic_hook
```

## index.html

프로젝트 root에 기본 html 태그를 구성한 index.html 을 만들어줘야한다.

```html
<!DOCTYPE html>
<html>
  <head></head>
  <body></body>
</html>
```

또는

```rust
<!DOCTYPE html>
<html lang="ko">
  <head>
    <meta charset="utf-8"/>
    <meta name="viewport" content="width=device-width, initial-scale=1"/>
    <title>Home</title>
    <link data-trunk rel="rust" data-wasm-opt="z"/>
  </head>
  <body></body>
</html>
```

- `<link data-trunk rel="rust" data-wasm-opt="z"/>` 에 대해서는 각자 알아 볼 것

## main.rs

leptos::prelude::* 를 추가하고  mount 코드를 추가해준다.

```rust
use leptos::prelude::*;

fn main() {
		console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(|| view! { <p>"Hello, world!"</p> })
}
```

## 서버 구동

```bash
$ trunk serve --open
```