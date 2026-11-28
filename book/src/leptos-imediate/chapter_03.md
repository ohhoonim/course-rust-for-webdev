# Iteration과 Control Flow

## 목표

- Leptos에서 정적 목록(`Vec<_>`)을 렌더링하는 방법
- `<For>` 컴포넌트를 사용하여 동적 목록 렌더링 하기
- `<Show>` 컴포넌트를 사용한 조건별 새로고침 최적화 방법 활용하기 

---

## 상황

- Todo: 할 일을 등록하고 목록을 등록삭제 하는 컴포넌트
- Login Gate: 로그인 정보를 입력하고 아이디(이메일) validation을 수행하고, 검증 완료시 welcome 정보를 보여주는 간단한 컴포넌트. welcome에서는 logout을 form에서는 id, password를 표시하고 login 버튼을 보여준다.

## 힌트

```rust
// todo struct
pub struct Todo {
    id: u64,
    title: String,
}

// email validation
email.read().contains('@') && pasword.read().len() > 4
```

### `RwSignal`과 Form

`signal()` 을 사용하면 읽기와 쓰기 변수, 두 개를 사용하므로 Form요소에 사용하기 불편하다. 이럴때 `signal()` 보다는 `RwGignal`을 사용하면 하나의 변수로 처리되므로  편리하다.

```rust

let email = RwSignal::new(String::new());

view! {
    <input
        type="email"
        placeholder="email"
        prop:value=email
        on:input:target=move |event| email.set(event.target().value())
    />
}


```

### event.prevent_default()

form의 submit, a 태그와 같이 기본 버블링 동작이 있는 경우 leptos에서도 javascript로 작성할때와 마찬가지로 기본동작을 막아주어야 한다. 

```rust
view! {
    <form class="stack" on:submit=move |event| {
        event.prevent_default();
        if email.read().contains('@') && password.read().len() >= 4 {
            logged_in.set(true);
        }
    }>

    </form>
}
```

---

### `<For>` 컴포넌트

```rust
let todos = RwSignal::new(vec![
        Todo {
            id: 1,
            title: "Render a keyed list".into(),
        },
        Todo {
            id: 2,
            title: "Delete one row without rebuilding the rest".into(),
        },
    ]);

view!{
   <ul class="list">
       <For
           each=move || todos.get()
           key=|todo| todo.id
           children=move |todo| {
               view! {
                   <li >
                       <span>>{todo.id}> " : " {todo.title}</span>
                   </li>
               }
           }
       />
   </ul>
}
```

#### let() 문법

데이터만 확인해보고 싶다면 ‘let(child)’ 문법을 사용할 수 있다. 

```rust
children=|child| view! { <p>{child.value}</p> }
// 이렇게 사용하거나 
// 아래와 같이 let 구문을 이용할 수 있다.
<For ..생략
let(child) />
    <p>{child.value}</p>
</For>
// 다음과 같이 구조 분해를 사용할 수도 있다. 
let(DatabaseEntiry {key, value} // DatabaseEntry struct
 
```

#### `<For />` 컴포넌트 사용시 주의 사항

Leptos의 <For/> 컴포넌트는 성능 최적화를 위해 "고유한 Key가 변경되었을 때만" 해당 행(Row)을 다시 렌더링(Rerender)하도록 설계되어 있다. 다음의 코드를 실행해보자. 자세한 내용은 [Iterating over More Complex Data with <For/>](https://book.leptos.dev/view/04b_iteration.html) 문서를 참고

```rust
#[component]
pub fn App() -> impl IntoView {
    // start with a set of three rows
    let (data, set_data) = signal(vec![
        DatabaseEntry {
            key: "foo".to_string(),
            value: 10,
        },
        DatabaseEntry {
            key: "bar".to_string(),
            value: 20,
        },
        DatabaseEntry {
            key: "baz".to_string(),
            value: 15,
        },
    ]);
    view! {
        // when we click, update each row,
        // doubling its value
        <button on:click=move |_| {
            set_data.update(|data| {
                for row in data {
                    row.value *= 2;
                }
            });
            // log the new value of the signal
            leptos::logging::log!("{:?}", data.get());
        }>
            "Update Values"
        </button>
        // iterate over the rows and display each value
        <For
            each=move || data.get()
            key=|state| state.key.clone()
            let(child)
        >
            <p>{child.value}</p>
        </For>
    }
}
```

- `signal`의 값 자체는 정상적으로 수정되고, 로그를 찍어보면 `value`가 2배로 바뀐 것이 출력됩니다.
- 하지만 화면(DOM)상에서는 **`<p>{child.value}</p>`로 렌더링된 값들이 전혀 바뀌지 않고 그대로 유지**됩니다. (즉, 화면이 갱신되지 않습니다.)

### `<Show>` 컴포넌트

```rust
let logged_in = RwSignal::new(false);

view!{
    <Show
        when=move || logged_in.get()
        fallback=move || view! {
            <!-- when 조건이 거짓 일 때  화면 -->
        }
    >
        <!-- when 조건이 참 일 때 화면 -->
    </Show>
}
```

---

## 결과

- Todo Component

```rust
#[derive(Clone)]
pub struct Todo {
    id: u64,
    title: String,
}

#[component]
pub fn TodoList() -> impl IntoView {
    let draft = RwSignal::new(String::new());
    let todos = RwSignal::new(vec![
        Todo {
            id: 1,
            title: "Render a keyed list".into(),
        },
        Todo {
            id: 2,
            title: "Delete one row without rebuilding the rest".into(),
        },
    ]);

    let next_id = RwSignal::new(3_u64);

    let add_todo = move || {
        let title = draft.read().trim().to_string();
        if title.is_empty() {
            return;
        }
        let id = next_id.get();
        next_id.set(id + 1);
        todos.update(|items| items.push(Todo { id, title }));
        draft.set(String::new());
    };

    view! {
        <section class="panel">
            <h2>"TodoList - keyed <For/>"</h2>
            <form class="row" on:submit=move |event| {
                event.prevent_default();
                add_todo();
            }>
                <input
                    placeholder="New todo"
                    prop:value=draft
                    on:input:target=move |event| draft.set(event.target().value())
                />
                <button type="submit">"Add"</button>
            </form>
            <ul class="list">
                <For
                    each=move || todos.get()
                    key=|todo| todo.id
                    children=move |todo| {
                        let id = todo.id;
                        view! {
                            <li class="row between">
                                <span><code>{format!("#{}", todo.id)}</code> " " {todo.title}</span>
                                <button class="danger" on:click=move |_| {
                                    todos.update(|items| items.retain(|item| item.id != id));
                                }>"Delete"</button>
                            </li>
                        }
                    }
                />
            </ul>
        </section>
    }
}

```

- Login Gate 

```rust
#[component]
pub fn LoginGate() -> impl IntoView {
    let logged_in = RwSignal::new(false);
    let email = RwSignal::new(String::new());
    let password = RwSignal::new(String::new());

    view! {
        <section class="panel">
            <h2>"LoginGate - <Show/>"</h2>
            <Show
                when=move || logged_in.get()
                fallback=move || view! {
                    <form class="stack" on:submit=move |event| {
                        event.prevent_default();
                        if email.read().contains('@') && password.read().len() >= 4 {
                            logged_in.set(true);
                        }
                    }>
                        <input
                            type="email"
                            placeholder="email"
                            prop:value=email
                            on:input:target=move |event| email.set(event.target().value())
                        />
                        <input
                            type="password"
                            placeholder="password"
                            prop:value=password
                            on:input:target=move |event| password.set(event.target().value())
                        />
                        <button type="submit">"Log in"</button>
                    </form>
                }
            >
                <div class="success">
                    <strong>"Welcome back."</strong>
                    <p>{move || format!("Signed in as {}", email.get())}</p>
                    <button on:click=move |_| logged_in.set(false)>"Log out"</button>
                </div>
            </Show>
        </section>
    }
}
```
---

## 과제

- Tab panel
- User Card
- Safe Divider

## 과제 풀이

`b04-control-flow/src/components` 참고 
