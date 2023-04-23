mod api;
mod components;
use components::{create_form::CreateForm, table::GoLinksTable};
use serde::Deserialize;
use stylist::yew::use_style;
use wasm_bindgen::UnwrapThrowExt;
use web_sys::window;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct GoLink {
    id: String,
    name: String,
    target: String,
    #[serde(rename = "createdAt")]
    created_at: String,
    #[serde(rename = "updatedAt")]
    updated_at: String,
    visits: i32,
}

#[function_component]
fn Dashboard() -> Html {
    let location = window().expect_throw("window is undefined").location();
    let hostname = location.hostname().unwrap();

    let golinks = use_state(|| vec![]);

    let fetch_all_golinks = {
        let golinks = golinks.clone();
        Callback::from(move |_| {
            let golinks = golinks.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let fetched_golinks: Vec<GoLink> = api::get_all_golinks().await.unwrap();
                golinks.set(fetched_golinks);
            });
        })
    };

    {
        let fetch_all_golinks = fetch_all_golinks.clone();
        use_effect_with_deps(
            move |_| {
                fetch_all_golinks.emit(());
                || ()
            },
            (),
        );
    }

    let container_style = use_style!(
        r#"
            margin: 0 auto;
            max-width: 85rem;
            display:flex;
            align-items:center;
            flex-direction:column;
        "#
    );
    let title_style = use_style!("margin-top:40px;");

    html! {
        <div class={container_style}>
            <title>{&hostname}{"/links"}</title>
            <h1>{&hostname}{"/links"}</h1>
            <CreateForm hostname={hostname} refetch={&fetch_all_golinks} />
            <h3 class={title_style}>{"Your Links"}</h3>
            <GoLinksTable golinks={(*golinks).clone()} refetch={&fetch_all_golinks} />
        </div>
    }
}

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/links")]
    Dashboard,
    #[at("/")]
    DashboardRedirect,
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Dashboard => html! { <Dashboard /> },
        Route::DashboardRedirect => html! { <Redirect<Route> to={Route::Dashboard} /> },
    }
}

#[function_component]
fn App() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
