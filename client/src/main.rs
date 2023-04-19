mod api;
use serde::Deserialize;
use wasm_bindgen::JsCast;
use wasm_bindgen::UnwrapThrowExt;
use web_sys::{window, EventTarget, HtmlInputElement};
use yew::prelude::*;

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
    let name = use_state(|| "".to_string());
    let target = use_state(|| "".to_string());

    let fetch_all_golinks = {
        let golinks = golinks.clone();
        || {
            wasm_bindgen_futures::spawn_local(async move {
                let fetched_golinks: Vec<GoLink> = api::get_all_golinks().await;
                golinks.set(fetched_golinks);
            });
        }
    };

    {
        let fetch_all_golinks = fetch_all_golinks.clone();
        use_effect_with_deps(
            move |_| {
                fetch_all_golinks();
                || ()
            },
            (),
        );
    }

    let name_on_input: Callback<InputEvent> = {
        let name = name.clone();
        Callback::from(move |e: InputEvent| {
            let target: Option<EventTarget> = e.target();
            let input = target.and_then(|t| t.dyn_into::<HtmlInputElement>().ok());

            if let Some(input) = input {
                name.set(input.value());
            }
        })
    };

    let target_on_input = {
        let target_handle: UseStateHandle<String> = target.clone();
        Callback::from(move |e: InputEvent| {
            let target: Option<EventTarget> = e.target();
            let input: Option<HtmlInputElement> =
                target.and_then(|t: EventTarget| t.dyn_into::<HtmlInputElement>().ok());

            if let Some(input) = input {
                target_handle.set(input.value());
            }
        })
    };

    let create_golink = {
        let name = name.clone();
        let target: UseStateHandle<String> = target.clone();
        let fetch_all_golinks = fetch_all_golinks.clone();
        move || {
            let name_string = name.to_string();
            let target_string = target.to_string();
            let fetch_all_golinks = fetch_all_golinks.clone();
            wasm_bindgen_futures::spawn_local(async move {
                api::create_golink(name_string, target_string).await;
                fetch_all_golinks();
            });
        }
    };

    let handle_go_click = {
        let create_golink = create_golink.clone();
        Callback::from(move |_| create_golink())
    };

    let handle_delete_click = |name: &String| -> yew::Callback<web_sys::MouseEvent> {
        let name = name.clone();
        let fetch_all_golinks = fetch_all_golinks.clone();
        Callback::from(move |_| {
            let name_string = name.to_string();
            let fetch_all_golinks = fetch_all_golinks.clone();
            wasm_bindgen_futures::spawn_local(async move {
                api::delete_golink(&name_string).await;
                fetch_all_golinks();
            });
        })
    };

    let on_key_down = {
        let create_golink = create_golink.clone();
        Callback::from(move |e: KeyboardEvent| {
            if e.key() == "Enter" {
                create_golink()
            }
        })
    };

    html! {
        <div style="margin: 0 auto;max-width: 80rem;display:flex;align-items:center;flex-direction:column;">
            <title>{&hostname}{"/links"}</title>
            <h1>{&hostname}{"/links"}</h1>
            <div style="display:flex;width:100%;gap:10px;align-items:center;">
                <span>{&hostname}{"/"}</span>
                <input style="margin:0" type="text" placeholder="name" oninput={name_on_input} onkeydown={&on_key_down} />
                <span>{"→"}</span>
                <input style="margin:0" type="text" placeholder="paste a url here..." oninput={target_on_input} onkeydown={&on_key_down} />
                <button style="margin:0" onclick={handle_go_click} >{"go!"}</button>
            </div>
            // TODO: request error <span style="color: red;">{"Create a link to any URL"}</span>
            <h3 style="margin-top:40px;">{"Your Links"}</h3>
            <table>
                <thead>
                    <tr>
                        <th>{"Name"}</th>
                        <th>{"URL"}</th>
                        <th>{"Actions"}</th>
                    </tr>
                </thead>
                <tbody>
                    {
                        golinks.iter().map(|golink| html! {
                        <tr>
                            <td>{&golink.name}</td>
                            <td>{&golink.target}</td>
                            <td>
                                <button class="button button-clear" onclick={handle_delete_click(&golink.name)}>{"Delete"}</button>
                            </td>
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
