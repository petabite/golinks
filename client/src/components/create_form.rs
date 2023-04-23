use crate::api;
use crate::components;
use components::input::Input;
use serde::Deserialize;
use stylist::yew::use_style;
use yew::prelude::*;
use yew_router::prelude::use_location;

#[derive(PartialEq, Properties)]
pub struct CreateFormProps {
    pub hostname: AttrValue,
    pub refetch: Callback<()>,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]

struct QueryParams {
    #[serde(default)]
    name: String,
}

#[function_component]
pub fn CreateForm(props: &CreateFormProps) -> Html {
    let location = use_location();
    let initial_name = {
        match location {
            Some(location) => location.query::<QueryParams>().unwrap().name,
            None => "".to_string(),
        }
    };
    let CreateFormProps { hostname, refetch } = props;
    let name = use_state(|| initial_name);
    let target = use_state(|| "".to_string());
    let error_message = use_state(|| "".to_string());
    let name_on_input = {
        let name = name.clone();
        Callback::from(move |value| {
            name.set(value);
        })
    };

    let target_on_input = {
        let target_handle = target.clone();
        Callback::from(move |value| {
            target_handle.set(value);
        })
    };

    let create_golink = {
        let name = name.clone();
        let target: UseStateHandle<String> = target.clone();
        let error_message = error_message.clone();
        let refetch = refetch.clone();
        move || {
            let name = name.clone();
            let target = target.clone();
            let error_message = error_message.clone();
            let refetch = refetch.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let response = api::create_golink((*name).clone(), (*target).clone()).await;
                match response {
                    Ok(_) => {
                        name.set("".to_string());
                        target.set("".to_string());
                        error_message.set("".to_string());
                        refetch.emit(());
                    }
                    Err(e) => {
                        error_message.set(e);
                    }
                }
            });
        }
    };

    let on_key_down = {
        let create_golink = create_golink.clone();
        Callback::from(move |e: KeyboardEvent| {
            let create_golink = create_golink.clone();
            if e.key() == "Enter" {
                create_golink()
            }
        })
    };

    let handle_go_click = {
        let create_golink = create_golink.clone();
        Callback::from(move |_| {
            let create_golink = create_golink.clone();
            create_golink()
        })
    };

    let container_style = use_style!(
        r#"
            display:flex;
            width:100%;
            gap:10px;
            align-items:center;
        "#
    );

    html! {
        <>
            <div class={container_style}>
                <span>{&hostname}{"/"}</span>
                <Input placeholder="name" value={(*name).clone()} oninput={name_on_input} onkeydown={&on_key_down} />
                <span>{"→"}</span>
                <Input placeholder="paste a url here..." value={(*target).clone()} oninput={target_on_input} onkeydown={&on_key_down} />
                <button style="margin:0" onclick={handle_go_click} >{"go!"}</button>
            </div>
            <span style="color: red;">{&*error_message}</span>
        </>
    }
}
