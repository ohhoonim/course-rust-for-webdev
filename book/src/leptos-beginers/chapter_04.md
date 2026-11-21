# 기본 구성요소 살펴보기

## #[component] (컴포넌트)

재사용 가능한 UI 조각을 정의하는 Rust 함수이다. 함수 위에 `#[component]` 매크로를 붙이면 Leptos가 이를 HTML 태그처럼 사용할 수 있는 컴포넌트로 변환한다.

- 정의: `pub fn SimpleCounter(initial_value: i32, step: i32) -> impl IntoView`
- 사용: `main` 함수 내부의 `view! { <SimpleCounter initial_value="0" step="1"/> }`에서 커스텀 태그 형태로 호출됩니다.
- 반환 타입: `impl IntoView`를 반환해 렌더링 가능한 뷰임을 나타냅니다.

## Prop (속성, #[prop])

부모 컴포넌트(여기서는 `main`의 `view!`)에서 자식 컴포넌트(`SimpleCounter`)로 데이터를 전달할 때 사용하는 함수 매개변수이다.

- 정의: `initial_value: i32`와 `step: i32`가 `SimpleCounter`의 prop이다.
- 전달: `<SimpleCounter initial_value="0" step="1"/>`처럼 HTML 속성 문법으로 값을 넘겨준다
- 역할: `initial_value`는 카운터의 초기값을, `step`은 버튼을 누를 때 증감할 단위를 결정한다.

## signal() (시그널)

Leptos에서 반응형 상태(Reactive State)를 관리하는 핵심 primitive이다. 시그널 값이 바뀌면 그 값을 구독 중인 UI 부분만 정밀하게 다시 렌더링한다.

- 생성: `let (value, set_value) = signal(initial_value);`
    - `value` (읽기 핸들 / ReadSignal): 현재 카운트 값을 읽는다.
    - `set_value` (쓰기 핸들 / WriteSignal): 값을 변경할 때 사용한다다.
- 상태 변경 방식:
    - `set_value.set(0)`: `"Clear"` 버튼 클릭 시 값을 `0`으로 직접 설정한다.
    - `*set_value.write() -= step`: `"-1"` 버튼 클릭 시 내부 값에 가변 참조(`&mut`)로 접근해 `step`만큼 뺀다
    - `set_value.update(|value| *value += step)`: `"+1"` 버튼 클릭 시 클로저로 현재 값을 받아 `step`만큼 더한다

## view!  (매크로)

Rust 코드 안에서 HTML과 유사한 선언적 문법으로 UI 트리와 이벤트 핸들러, 반응형 바인딩을 작성하는 매크로.

- HTML 구조와 텍스트: `<div>`, `<button>`, `<span>` 태그를 사용하고, 문자열 리터럴(`"Clear"`, `"Value: "` 등)을 배치합니다.
- 이벤트 리스너: `on:click=move |_| ...` 형태로 클릭 이벤트를 연결
- 반응형 바인딩: `<span>"Value: " {value} "!"</span>`에서 `{value}`처럼 중괄호로 시그널을 감싸면, `value`가 변경될 때 해당 텍스트 노드만 자동으로 업데이트된다.