use leptos::prelude::*;


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