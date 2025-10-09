use leptos::prelude::*;
use leptos_router::{
    components::{Route, Router, Routes},
    hooks::use_url,
    path,
};

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <AutoReload options=options.clone()/>
                <HydrationScripts options=options islands=true islands_router=true/>
                <link rel="stylesheet" id="leptos" href="/pkg/islands.css"/>
                <link rel="shortcut icon" type="image/ico" href="/favicon.ico"/>
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
        <Router>
            <main>
                <Routes fallback=|| "Not found.">
                    <Route path=path!("") view=B/>
                    <Route path=path!("b") view=A/>
                </Routes>
            </main>
        </Router>
        <nav>
            <ul>
                <li>
                    <a
                        class=("menu-active", move || use_url()().path() == "/")
                        href="/"
                    >
                        Home
                    </a>
                </li>
                <li>
                    <a
                        class=("menu-active", move || use_url()().path() == "/b")
                        href="/b"
                    >
                        CutImage
                    </a>
                </li>
            </ul>
        </nav>
    }
}

#[component]
pub fn A() -> impl IntoView {
    view! { <a href="/">a</a> }
}

#[component]
pub fn B() -> impl IntoView {
    view! { <a href="/b">b</a> }
}