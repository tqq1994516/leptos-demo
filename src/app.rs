use leptos::{prelude::*, server_fn::codec::Postcard, logging::debug_warn};
use leptos_router::{
    components::{Route, Router, Routes, ParentRoute, Outlet},
    path,
};

#[derive(Debug, Clone)]
pub struct Ctx(pub String);

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <AutoReload options=options.clone()/>
                <HydrationScripts options=options/>
                <link rel="stylesheet" id="leptos" href="/pkg/demo1.css"/>
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
                    <ParentRoute path=path!("") view=Layout>
                        <Route path=path!("") view=A/>
                        <Route path=path!("/project-manager/:id") view=B />
                        <ParentRoute path=path!("/project-manager/:id/") view=SubLayout>
                            <Route path=path!("user-manager") view=C/>
                        </ParentRoute>
                    </ParentRoute>
                </Routes>
            </main>
        </Router>
    }
}

#[component]
pub fn Layout() -> impl IntoView {
    view! { <Outlet /> }
}

#[component]
pub fn SubLayout() -> impl IntoView {

    let data = Resource::new(move || (), move |()| project_list());

    let projects = move || {
        Suspend::new(async move {
            data
                .await
                .map(|value| view! {
                    <p>{value}</p>
                })
        })
    };
    view! {
        <Suspense fallback=move || view! { <div class="loading"></div> }>{projects}</Suspense>
        <Outlet />
    }
}

#[component]
pub fn A() -> impl IntoView {
    view! { <a href="/project-manager/1">b</a> }
}

#[component]
pub fn B() -> impl IntoView {
    let ctx = expect_context::<Ctx>();
    debug_warn!("ctx:{ctx:#?}");

    let data = Resource::new(move || (), move |()| project_list());

    let projects = move || {
        Suspend::new(async move {
            data
                .await
                .map(|value| view! {
                    <p>{value}</p>
                })
        })
    };

    view! {
        <a href="/project-manager/1/user-manager">c</a>
        <Suspense fallback=move || view! { <div class="loading"></div> }>{projects}</Suspense>
    }
}

#[component]
pub fn C() -> impl IntoView {
    let ctx = expect_context::<Ctx>();
    debug_warn!("ctx:{ctx:#?}");

    let data = Resource::new(move || (), move |()| project_list());

    let projects = move || {
        Suspend::new(async move {
            data
                .await
                .map(|value| view! {
                    <p>{value}</p>
                })
        })
    };

    view! {
        <a href="/">a</a>
        <Suspense fallback=move || view! { <div class="loading"></div> }>{projects}</Suspense>
    }
}

#[server(
    input = Postcard,
    output = Postcard,
)]
#[lazy]
pub(crate) async fn project_list() -> Result<String, ServerFnError> {
    let ctx = expect_context::<Ctx>();

    debug_warn!("ctx:{ctx:#?}");

    Ok(String::from("test1"))
}
