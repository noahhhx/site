use leptos::prelude::*;
use leptos_meta::Title;

use crate::app::Prompt;

/// Rendered from `content/cv.md` by `build.rs`.
const CV_HTML: &str = include_str!(concat!(env!("OUT_DIR"), "/cv.html"));

#[component]
pub fn Cv() -> impl IntoView {
    view! {
        <Title text="cv · noah"/>
        <div class="cv-toolbar">
            <Prompt command="cat cv.md"/>
            <button on:click=|_| {
                let _ = window().print();
            }>"export pdf"</button>
        </div>
        <article class="cv" inner_html=CV_HTML></article>
    }
}
