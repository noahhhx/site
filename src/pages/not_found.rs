use leptos::prelude::*;
use leptos_meta::Title;
use leptos_router::{components::A, hooks::use_location};

use crate::app::Prompt;

#[component]
pub fn NotFound() -> impl IntoView {
    #[cfg(feature = "ssr")]
    if let Some(response) = use_context::<leptos_axum::ResponseOptions>() {
        response.set_status(axum::http::StatusCode::NOT_FOUND);
    }

    let path = use_location().pathname;

    view! {
        <Title text="not found · noah"/>
        <Prompt command="cd"/>
        <p>
            <span class="fg-red">"cd: no such file or directory: "</span>
            {move || path.get()}
        </p>
        <p><A href="/">"cd ~"</A></p>
    }
}
