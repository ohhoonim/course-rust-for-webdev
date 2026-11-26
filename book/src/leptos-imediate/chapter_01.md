# Component

## 목표

- **컴포넌트(Components)**: 프론트 프레임워크의 필수 개념인 컴포넌트를 Leptos에서 사용하는 방법을 안다.
- **props**: html 요소에 속성을 주듯 컴포넌트 속성으로 데이터를 전달하는 방법과 prop drilling을 이해한다.
- **children**: 자식 컴포넌트를 전달하는 두 가지 방법과 slot 사용법을 알아본다.

---

## 상황

- variant 적용이 가능한 버튼 컴포넌트
- variant 버튼을 유형별로 보여주는 Card 컴포넌트 

## 힌트

```rust
pub enum ButtonVariant {
    Primary,
    Secondary,
    Danger,
    Ghost,
}
```

---

### props

- `#[prop(optional)]`: 필수가 아닌 선택적 props로 지정합니다. 값을 넘기지 않으면 해당 타입의 기본값(`Default::default()`)이 적용됩니다. `Option<>`을 사용하십시오.
- `#[prop(default = ...)]`: 기본값을 지정할 수 있습니다. (예: `#[prop(default = 100)] max: u16`)
- `#[prop(into)]`: 전달받은 값에 자동으로 `.into()`를 호출해 줍니다. 이를 통해 `ReadSignal` 같은 다양한 반응형 타입을 범용적인 `Signal` 타입으로 쉽게 변환하여 받을 수 있습니다.

#### - into prop에 대한 보충 설명 

`#[prop(into)]`를 붙이면, 사용자가 어떤 형태의 값(signal, 파생 signal 등)을 넣든 자동으로 .into()를 호출하여 컴포넌트가 원하는 통일된 타입(예: Signal<i32>)으로 변환해 줍니다.

```
[문제 상황]
다양한 신호나 클로저를 컴포넌트에 전달하고 싶음!
       │
       ▼
1. Generic Props (제네릭) ──> 함수나 클로저를 받을 수 있게 열어줌 (하지만 타입 정의가 복잡해짐)
       │
       ▼
2. Marker Props (마커)   ──> 제네릭 사용 시 발생하는 불필요한 메타데이터/문서를 깔끔하게 정리해 줌
       │
       ▼
3. Into Props (최종 완성)  ──> #[prop(into)]를 통해 알아서 `.into()`로 타입을 변환해주어,
                              복잡한 제네릭 없이도 다양한 데이터를 가장 깔끔하게 받음
```

#### - 속성 전개(Attribute Spreading)

- 부모 컴포넌트에서 전달한 HTML 속성(예: `class`, `id`, `aria-label` 등)을 자식 컴포넌트 내부의 최상위 요소로 그대로 전달(스프레드)할 수 있습니다.
- `{..}` 문법을 사용하여 처리합니다.

```rust
#[component]
fn SubmitButton(
    #[prop(into)] label: String,
) -> impl IntoView {
    view! {
        <button
            class="base-btn px-4 py-2 rounded font-semibold bg-blue-500 text-white"
            {..}
        >
            {label}
        </button>
    }
}
```

#### - props Drilling

컴포넌트가 깊게 중첩되어 있어 중간 컴포넌트들을 거쳐 Prop을 계속 내려줘야 하는 'Prop Drilling' 현상이 발생 할 수도 있다. 부모 컴포넌트에서 자식 컴포넌트로 데이터를 전달하는 것은 Props(ReadSignal이나 Signal)를 통해 쉽지만, 반대로 자식 컴포넌트에서 부모에게 이벤트나 상태 변경을 알리는 방법은 몇 가지 패턴이 존재한다.

- `WriteSignal`을 prop으로 전달
- 콜백함수를 prop으로 전달
- 이벤트리스너 활용
- context API 사용

상세내용은 [Parent-Child Communication](https://book.leptos.dev/view/08_parent_child.html)을 참고

---
### children

Leptos에서 컴포넌트에 내용을 전달하는 방식은 크게 두 가지가 있습니다. 더 자세한 내용은 [Passing Children to Components](https://book.leptos.dev/view/09_component_children.html)을 참고하십시오.

- **Render Props (렌더 프로퍼티):** <u>**뷰(View)를 반환하는 함수**</u>를 컴포넌트의 일반 속성(Prop)으로 전달하는 방식입니다.
- **Children Prop (`children` 속성):** 컴포넌트 태그 사이에 작성한 모든 내용을 자동으로 받아오는 특별한 속성입니다.

예를 들어, Show 컴포넌트를 보면:

```rust
view! {
  <Show
    when=move || value.get() > 5
    fallback=|| view! { <Small/> }
  >
    <Big/>
  </Show>
}
```

- `fallback`은 **렌더 프로퍼티**(함수를 실행해 뷰를 만듦)로 사용되고,
- 컴포넌트 태그 내부에 있는 `<Big/>` 같은 자식들은 **`children` 속성**으로 전달된다.


#### - 슬롯 (Slots): 타입이 지정된 자식 

단순히 `children` 하나만 받는 것을 넘어, 여러 종류의 자식을 구조화해서 받고 싶을 때 슬롯(Slots)을 사용한다.

- `#[slot]` 매크로를 이용해 구조체를 정의하면, 부모 컴포넌트 안에서 `<Then slot:then_slot>...</Then>` 같은 형태로 특정 자식의 위치를 명시한다.
- 굳이 이름을 붙이지 않고 `<Then slot>`처럼 쓰면 구조체 이름의 스네이크 케이스(`then`)를 기본값으로 사용합니다.

```rust
use std::time::Duration;

#[slot]
struct HelloSlot {
    // Same prop syntax as components.
    #[prop(optional)]
    children: Option<Children>,
}

#[component]
fn HelloComponent(
    /// Component slot, should be passed through the <HelloSlot slot> syntax.
    hello_slot: HelloSlot,
) -> impl IntoView {
    hello_slot.children.map(|children| children())
}

#[component]
fn App() -> impl IntoView {
    view! {
        <HelloComponent>
            <HelloSlot slot>
                "Hello, World!"
            </HelloSlot>
        </HelloComponent>
    }
}
```




---

## 결과

### Button 컴포넌트

```rust
#[derive(Clone, PartialEq)]
pub enum ButtonVariant {
    Primary,
    Secondary,
    Danger,
    Ghost,
}

#[component]
pub fn Button(
    #[prop(into)] label: String,
    #[prop(default = ButtonVariant::Primary)] variant: ButtonVariant,
    #[prop(default = false)] disabled: bool,
    #[prop(optional)] on_click: Option<Callback<web_sys::MouseEvent>>,
) -> impl IntoView {
    let (bg, text_color) = match &variant {
        ButtonVariant::Primary => ("#1d4ed8", "white"),
        ButtonVariant::Secondary => ("#6b7280", "white"),
        ButtonVariant::Danger => ("#dc2626", "white"),
        ButtonVariant::Ghost => ("transparent", "#374151"),
    };

    view! {
        <button class="btn-command"
            style:background=bg
            style:color=text_color
            style:border-color=bg
            disabled=disabled
            on:click=move |event| {
                if let Some(callback) = &on_click {
                    callback.run(event);
                }
            }
        >
            {label}
        </button>
    }
}

```

### Card 컴포넌트 

```rust
#[component]
pub fn Card(
    #[prop(into)] title: String,
    #[prop(optional)] subtitle: Option<String>,
    children: Children,
) -> impl IntoView {
    view! {
        <div class="card">
            <div class="heading">
                <h3>{title}</h3>
                {subtitle.map(|s| view! { <p>{s}</p> })}
            </div>
            <div class="section">{children()}</div>
        </div>
    }
}
```

---

## 과제

### alert

다음의 Alert type을 지원하는 alert 컴포넌트를 작성한다.

```rust
pub enum AlertType {
    Info,
    Success,
    Warning,
    Error,
}
```

### avatar

다음의 Avatar size를 지원하는 컴포넌트를 작성한다. 이름에서 앞 두 글자만 표시한다. 

- sm: 32px, 0.75rem
- md: 48px, 1rem
- lg: 64px, 1.25rem

```rust
pub enum AvatarSize {
    Sm,
    Md,
    Lg,
}
```


### badge

색상으로 라벨을 표시하는 뱃지 컴포넌트를 작성한다.

```rust
pub enum BadgeColor {
    Blue,
    Green,
    Yellow,
    Red,
    Gray,
}
```

## 과제 풀이

`b02-component/src/components` 참고 