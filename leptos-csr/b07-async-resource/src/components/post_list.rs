use leptos::prelude::*;

use crate::models::fetch_posts;



#[component]
fn Skeleton() -> impl IntoView {
    view! {
        <div class="skeletons">
            <div></div><div></div><div></div>
        </div>
    }
}

/// Explains the `post list` step in this lesson.
///
/// What it does: renders the `PostList` component for one focused part of the lesson UI.
/// Why it matters: Keeping this UI section isolated makes the signal, context, action, or routing pattern easier to study.
#[component]
pub fn PostList() -> impl IntoView {
    let posts = LocalResource::new(fetch_posts);
    let refresh_count = RwSignal::new(0_u32);

    Effect::new(move |_| {
        if matches!(posts.get(), Some(Ok(_))) {
            refresh_count.update(|count| *count += 1);
        }
    });

    view! {
        <section class="panel">
            <h2>"PostList - Resource + Suspense"</h2>
            <div class="row">
                <button on:click=move |_| posts.refetch()>"Refetch posts"</button>
                <span>{move || format!("successful refreshes: {}", refresh_count.get())}</span>
            </div>
            <Suspense fallback=Skeleton>
                {move || posts.get().map(|result| match result {
                    Ok(posts) => view! {
                        <div class="cards">
                            {posts.into_iter().map(|post| view! {
                                <article>
                                    <strong>{format!("Post {}", post.id)}</strong>
                                    <h3>{post.title}</h3>
                                    <p>{post.body}</p>
                                </article>
                            }).collect_view()}
                        </div>
                    }.into_any(),
                    Err(error) => view! { <p class="error">{error}</p> }.into_any(),
                })}
            </Suspense>
        </section>
    }
}