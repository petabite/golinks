use yew::prelude::*;
use serde::Deserialize;
use gloo_net::http::Request;

#[derive(Debug, Clone, PartialEq, Deserialize)]
struct GoLink {
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
    let golinks = use_state(|| vec![]);
    {
        let golinks = golinks.clone();
        use_effect_with_deps(move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                let fetched_golinks: Vec<GoLink> = Request::get("http://127.0.01:9888/link")
                    .send()
                    .await
                    .unwrap()
                    .json()
                    .await
                    .unwrap();
                golinks.set(fetched_golinks);
            });
            || ()
        } , ());
    }

    html! {
        <div style="margin: 0 auto;max-width: 80rem;">
            <h1>{"go/links"}</h1>
            <input type="text" placeholder="name" />
            <input type="text" placeholder="paste a URL here..." />
            <button>{"create"}</button>
            <table>
                <thead>
                    <tr>
                        <th>{"Name"}</th>
                        <th>{"Target"}</th>
                        <th>{"Total Visits"}</th>
                    </tr>
                </thead>
                <tbody>
                    {
                        golinks.iter().map(|golink| html! {
                        <tr>
                            <td>{&golink.name}</td>
                            <td>{&golink.target}</td>
                            <td></td>
                        </tr>
                    }).collect::<Html>()}
                </tbody>
            </table>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
