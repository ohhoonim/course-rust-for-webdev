# Reactivity

## 목표

- Signal:  상태(데이터)의 변화를 추적하고, 값이 바뀔 때 그 값을 사용하는 UI 부분만 정확히 업데이트해 주는 반응형(Reactive) 변수 signal 사용법을 익힌다.
- Memo: 다른 시그널(Signal)의 값을 기반으로 계산된 결과를 캐싱하는 방법을 알아본다.
- Effects : 시그널 값이 변경될 때 특정 코드를 자동으로 실행할 수 있다. 

---

## 상황

- Counter 컴포넌트: 하나의 버튼으로 카운트 값을 1 증가와 2배 증가시키는 라벨을 출력하는 counter 컴포넌트. couunt 값이 바뀌면 devtool console에 로그를 출력한다. 
- Word Counter: textarea에 입력된 값을 실시간으로 단어수, 문자수, 중복제외 단어수를 표시하는 Word Counter 컴포넌트. 단어는 공백으로 구분한다.

## 힌트

```rust
// leptos logging 사용하기 
leptos::logging::log!("{}", "hello");

// leptos에서 input event 사용
<textarea
    placeholder="텍스트를 입력하세요..."
    prop:value=text
    on:input:target=move |event| set_text.set(event.target().value())
/></text>
```

### prop(property)과 attr(attribute) 구분하기

leptos에서는 prop과 attr을 구분하기를 권장한다. Attribute는 HTML 마크업에 정의된 초기 속성이고, Property는 브라우저가 HTML을 파싱해 만든 DOM 객체의 속성이다. 따라서 `prop:` 을 다음과 같이 사용할 수 있다.

```rust
<textarea
    prop:value=text
```


---
### Signal

signal() 함수를 통해 생성하며, 값을 읽는 Getter(ReadSignal)와 값을 수정하는 Setter(WriteSignal) 쌍으로 나뉜다.

```rust
let (count, set_count) = signal(0);
```

- **파생 시그널(Derived Signal)**: 기존 시그널(Signal)의 값을 읽어 가공하거나 계산한 결과를 반환하는 일반 클로저. (시그널을 이용하는 클로저)

### Memo와 파생 시그널(derived signal)과의 차이

- 파생 시그널 (move || count.get() * 2): 읽을 때마다 매번 계산이 실행된다. 가벼운 연산에 적합. 
- Memo (Memo::new(...)): 의존성이 바뀔 때 딱 한 번만 계산하고 결과를 저장. 무거운 연산이나, 여러 곳에서 동시에 참조하는 값, 또는 결과가 같을 때 렌더링을 막고 싶을 때 사용.

### Effects

시그널의 값이 변경될 때마다 특정 코드(로그 출력, 네트워크 요청, 수동 DOM 조작 등)를 자동으로 실행한다. (React의 useEffect와 유사)

```rust
Effect::new(move |_| {
    // 오래 걸리는 작업
})
```

---

## 결과

- Counter 컴포넌트

```rust
#[component]
pub fn Counter() -> impl IntoView {
    let (count, set_count) = signal(0_i32);
    let doubled = Memo::new(move |_| count.get() * 2);

    Effect::new(move |_| {
        leptos::logging::log!("count changed to {}", count.get());
    });

    view! {
        <section>
            <h3>"계수기 - signal + memo + effect"</h3>
            <p>"Count: " <strong>{count}</strong></p>
            <p>"Doubled (Memo): " <strong>{doubled}</strong></p>
            <button on:click=move |_| set_count.update(|n| *n += 1)>"+"</button>
            <button  on:click=move |_| set_count.update(|n| *n -= 1)>"-"</button>
            <button  on:click=move |_| set_count.set(0)>"Reset"</button>
            <p>"Open DevTools console to see effect logs."</p>
        </section>
    }
}
```

- Word Counter 컴포넌트

```rust
#[component]
pub fn WordCounter() -> impl IntoView {
    let (text, set_text) = signal(String::new());
    let word_count = Memo::new(move |_| text.get().split_whitespace().count());
    let char_count = Memo::new(move |_| text.get().chars().count());
    let unique_words = Memo::new(move |_| {
        let current = text.get();
        let mut words: Vec<&str> = current.split_whitespace().collect();
        words.sort_unstable();
        words.dedup();
        words.len()
    });

    view! {
        <section>
            <h3>"단어 계수기 - multiple memos from one signal"</h3>
            <textarea
                placeholder="텍스트를 입력하세요..."
                prop:value=text
                on:input:target=move |event| set_text.set(event.target().value())
            />
            <p>"Words: " <strong>{word_count}</strong>
               " | Chars: " <strong>{char_count}</strong>
               " | Unique words: " <strong>{unique_words}</strong></p>
        </section>
    }
}
```
---

## 과제
- Temperature Converter: 섭씨 화씨 간 변환 계산기
    - 섭씨 -> 화씨: 섭씨 * 9.0 / 5.0 + 32.0
    - 화씨 -> 섭씨: (화씨 - 32.0) * 5.0 / 9.0
    - 화씨를 파생 시그널로 선언해보자. 
- Color Mixer: RGB 값을 받아 실시간으로 배경색을 변경해준다.
    - RGB 값 변경은 input[type="range"]를 사용
    - div의 배경색을 변경하고, hexcode를 텍스트로 보여준다.
- RwSiganl + batch() 을 이용하여 Effect 처리하는 코드를 이해하기

```rust
#[component]
pub fn RwSignalDemo() -> impl IntoView {
    let counter = RwSignal::new(0_i32);
    let (render_count, set_render_count) = signal(0_u32);

    Effect::new(move |_| {
        let _ = counter.get();
        set_render_count.update(|n| *n += 1);
    });

    view! {
        <section>
            <h3>"RwSignal + batch()"</h3>
            <p>"Counter: " <strong>{counter}</strong></p>
            <p>"Effect ran " {render_count} " time(s)"</p>
            <button on:click=move |_| counter.update(|n| *n += 1)>"+1"</button>
            <button
                on:click=move |_| {
                    batch(move || {
                        counter.update(|n| *n += 10);
                        counter.update(|n| *n += 10);
                        counter.update(|n| *n += 10);
                    });
                }
            >
                "+30 (batched)"
            </button>
            <button  on:click=move |_| counter.set(0)>"Reset"</button>
        </section>
    }
}
```

## 과제 풀이

`b03-reactivity/src/components` 참고 