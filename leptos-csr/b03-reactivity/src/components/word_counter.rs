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
        <section style="padding: 1rem; border: 1px solid #e2e8f0; border-radius: 8px; margin: 8px 0;">
            <h3>"Word Counter - multiple memos from one signal"</h3>
            <textarea
                style="width: 100%; height: 80px; padding: 8px; box-sizing: border-box;"
                placeholder="Type or paste text here..."
                prop:value=text
                on:input:target=move |event| set_text.set(event.target().value())
            />
            <p>"Words: " <strong>{word_count}</strong>
               " | Chars: " <strong>{char_count}</strong>
               " | Unique words: " <strong>{unique_words}</strong></p>
        </section>
    }
}