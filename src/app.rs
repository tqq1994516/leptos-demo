use leptos::prelude::*;
use leptos_router::{
    components::{Route, Router, Routes},
    Lazy, LazyRoute, lazy_route,
    path,
};
use serde::{Deserialize, Serialize};

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <AutoReload options=options.clone()/>
                <HydrationScripts options=options/>
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
                    <Route path=path!("") view={Lazy::<DataView>::new()}/>
                </Routes>
            </main>
        </Router>
    }
}

#[component]
pub(crate) fn ProjectTableHead() -> impl IntoView {
    view! {
        <thead>
            <tr>
                <th>
                    <label>
                        <input type="checkbox" class="checkbox" />
                    </label>
                </th>
                <th>Project Name</th>
                <th>Project Description</th>
                <th>Actions</th>
            </tr>
        </thead>
    }
}


#[component]
pub(crate) fn ProjectItem(
    id: String,
    name: String,
    desc: Option<String>,
    children: Children,
) -> impl IntoView {
    view! {
        <tr>
            <th>
                <label>
                    <input type="checkbox" class="checkbox" name="id" value=id />
                </label>
            </th>
            <td>
                <div class="flex items-center gap-3">
                    <div class="font-bold">{name}</div>
                </div>
            </td>
            <td class="max-w-xs truncate">
                <ShowLet some=move || desc.clone() let:desc fallback=move || ()>
                    <div class="tooltip" data-tip=desc>
                        {desc.clone()}
                    </div>
                </ShowLet>
            </td>
            <th>{children()}</th>
        </tr>
    }
}

#[derive(Serialize, Deserialize, Clone)]
struct Item {
    id: String,
    name: String,
    desc: Option<String>,
}

struct DataView {
    data: Resource<Result<Vec<Item>, ServerFnError>>,
}

#[lazy_route]
impl LazyRoute for DataView {
    fn data() -> Self {
        Self {
            data: Resource::new(|| (), move |_| data()),
        }
    }

    fn view(this: Self) -> AnyView {
        let items = move || {
            Suspend::new(async move {
                this
                    .data
                    .await
                    .map(|p| p
                        .into_iter()
                        .map(|item| view! {
                            {
                                let Item { id, name, desc } = item;
                                view! {
                                    <ProjectItem id name desc>
                                        <button
                                            class="btn btn-ghost btn-xs"
                                            on:click=move |_| leptos::logging::debug_warn!("111")
                                        >
                                            details
                                        </button>
                                    </ProjectItem>
                                }
                            }

                        })
                        .collect_view()
                    )
            })
        };
        view! {
            <div class="overflow-x-auto">
                <table class="table">
                    <ProjectTableHead />
                    <Suspense fallback=move || view! { <div class="loading"></div> }>
                        {items}
                    </Suspense>
                </table>
            </div>
        }.into_any()
    }
}



#[server]
#[lazy]
async fn data() -> Result<Vec<Item>, ServerFnError> {
    Ok(vec![
        Item {
            id: "1".to_string(),
            name: "Item 1".to_string(),
            desc: Some("This is item 1".to_string()),
        },
        Item {
            id: "2".to_string(),
            name: "Item 2".to_string(),
            desc: None,
        },
    ])
}