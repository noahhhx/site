use leptos::prelude::*;
use leptos_meta::{MetaTags, provide_meta_context};
use leptos_router::{
    components::{A, Route, Router, Routes},
    path,
};

use crate::pages::{Cv, Home, NotFound, Projects, Uses};

pub const GITHUB: &str = "https://github.com/noahhhx";
pub const EMAIL: &str = "noahhh@tuta.com";
pub const SIGNAL_HANDLE: &str = "@noahhh.01";
pub const SIGNAL: &str =
    "https://signal.me/#eu/RNtcYdXvy2W9uhWi5Jo9SvJbvdILdwUQf-I18MdAQSYPdVPNvxZFaaP293OIcpr9";

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <meta name="color-scheme" content="dark"/>
                <meta name="theme-color" content="#0f0f0f"/>
                <link rel="stylesheet" href="/pkg/site.css"/>
                <AutoReload options=options.clone()/>
                <HydrationScripts options/>
                <MetaTags/>
            </head>
            <body>
                <App/>
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Router>
            <main class="terminal">
                <Tabs/>
                <Routes fallback=NotFound>
                    <Route path=path!("/") view=Home/>
                    <Route path=path!("/projects") view=Projects/>
                    <Route path=path!("/uses") view=Uses/>
                    <Route path=path!("/cv") view=Cv/>
                </Routes>
            </main>
        </Router>
    }
}

/// Kitty-style tab bar; `<A>` marks the current page with `aria-current`.
#[component]
fn Tabs() -> impl IntoView {
    view! {
        <nav class="tabs">
            <A href="/" exact=true>"~"</A>
            <A href="/projects">"projects"</A>
            <A href="/uses">"uses"</A>
            <A href="/cv">"cv"</A>
        </nav>
    }
}

/// A link that leaves the site: opens in a new tab and gets a ↗ marker.
#[component]
pub fn Ext(href: &'static str, children: Children) -> impl IntoView {
    view! {
        <a class="ext" href=href target="_blank" rel="noopener noreferrer">
            {children()}
        </a>
    }
}

/// A shell prompt line, e.g. `❯ ls ~/projects`.
#[component]
pub fn Prompt(command: &'static str) -> impl IntoView {
    view! { <p class="prompt">{command}</p> }
}
