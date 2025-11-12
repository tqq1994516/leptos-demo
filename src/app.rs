use leptos::prelude::*;
use leptos_router::{
    components::{Route, Router, Routes},
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
                    <Route path=path!("") view=A/>
                </Routes>
            </main>
        </Router>
    }
}

#[island(lazy)]
pub fn A() -> impl IntoView {
    let add_todo_action = Action::new(|_: &String| {
        async { 
            std::thread::sleep(std::time::Duration::from_millis(250));
        }
    });
    let data = Resource::new(move || add_todo_action.version()(), move |_| data());
    view! {
        <button on:click=move |_| { add_todo_action.dispatch("".to_string()); }>add</button>
        <Suspense fallback=|| "Loading...">
            {
                move || Suspend::new(async move {
                    data.await.map(|data| {
                        view! {
                            <div>
                                "Data: " {data}
                            </div>
                        }
                    })
                })
            }
        </Suspense>
    }
}

#[server]
#[lazy]
async fn data() -> Result<Vec<i8>, ServerFnError> {
    Ok(vec![1, 2, 3])
}