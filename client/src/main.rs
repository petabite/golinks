mod api;
mod components;
use serde::Deserialize;
use wasm_bindgen::UnwrapThrowExt;
use web_sys::window;
use yew::prelude::*;

use components::{create_form::CreateForm, table::GoLinksTable};

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct GoLink {
    id: String,
    name: String,
    target: String,
    #[serde(rename = "createdAt")]
    created_at: String,
    #[serde(rename = "updatedAt")]
    updated_at: String,
}

#[function_component]
fn App() -> Html {
    let location = window().expect_throw("window is undefined").location();
    let hostname = location.hostname().unwrap();

    let golinks = use_state(|| vec![]);

    let fetch_all_golinks = {
        let golinks = golinks.clone();
        Callback::from(move |_| {
            let golinks = golinks.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let fetched_golinks: Vec<GoLink> = api::get_all_golinks().await;
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

    html! {
        <div style="margin: 0 auto;max-width: 80rem;display:flex;align-items:center;flex-direction:column;">
            <title>{&hostname}{"/links"}</title>
            <h1>{&hostname}{"/links"}</h1>
            <CreateForm hostname={hostname} refetch={&fetch_all_golinks} />
            <GoLinksTable golinks={(*golinks).clone()} refetch={&fetch_all_golinks} />
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
