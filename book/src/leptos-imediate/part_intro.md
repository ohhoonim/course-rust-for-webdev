# Leptos-CSR 2부 활용하기

## 목차

- **Components**: props, children
- **Reactivity**: signal, effect, memo
- **Iteration과 Control Flow**: Show, For
- **Forms and Events**: event, control input, validation
- **Routing**: Router, Routes, params
- **Async and Resource**: resource, Suspense, Transition
- **State Management**: local state, context, RwSignal
- **Styling**: static class, conditional class, dynamic style
- **Animation**: transition, keyfrome, deferred removal
- **Testing**: native test, WASM tesst, validation test
- **WASM Performance**: binary size, memo, batching
- **i18n and Accessibility**: translation, RTL, ARIA
- **JS Interop**: wasm-bindgen, web-sys, js-sys


## 부록: Leptos에서 정적 assets을 세팅하는 법(data-trunk)

`public/styles`와 `public/images`가 있다면, 두 폴더를 Trunk가 어떻게 처리하길 원하는지(스타일시트 파이프라인 vs 단순 정적 복사)에 따라 `data-trunk` 설정이 달라진다.

### 실습 권장 설정 (`index.html`)

CSS(`public/styles/...`)는 Trunk의 `rel="css"`로 처리해 빌드 파이프라인에 태우고, 이미지(`public/images`)는 `rel="copy-dir"`로 원본 그대로 복사하는 구성이 깔끔하다.

```html
<!DOCTYPE html>
<html lang="ko">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Leptos CSR</title>

    <link data-trunk rel="css" href="public/styles/main.css" />
    <link data-trunk rel="copy-dir" href="public/images" />

    <link data-trunk rel="rust" />
</head>
<body></body>
</html>
```

### 빌드 후 생성되는 경로와 Leptos 코드(`view!`)에서의 참조

- **`public/images` 복사 결과**: `data-target-path`를 따로 주지 않으면 기본적으로 **`dist/images/...`** 로 복사된다. 따라서 Leptos `view!`에서는 루트 기준 경로인 `/images/...`로 불러온다.
    
```rust
view! {
    <img src="/images/logo.png" alt="로고" />
}
```
    
- **`data-target-path`를 지정한 경우**: 예를 들어 `<link data-trunk rel="copy-dir" href="public/images" data-target-path="assets/images" />`로 적으면 `dist/assets/images/...`로 복사되므로 `<img src="/assets/images/logo.png" />`로 참조한다.
- **CSS 파일이 여러 개라면**: 필요한 CSS 파일마다 `<link data-trunk rel="css" href="public/styles/..." />`를 각각 적어 주면 된다.
