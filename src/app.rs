use leptos::prelude::*;

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
                <title>"noah"</title>
                <AutoReload options=options.clone()/>
                <HydrationScripts options/>
            </head>
            <body>
                <App/>
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    view! {
        <main class="terminal">
            <h1>"Hello, world!"</h1>
            <p class="prompt cursor"></p>
        </main>
    }
}
