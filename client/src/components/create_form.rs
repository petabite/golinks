use crate::api;
use crate::components;
use components::input::Input;
use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct CreateFormProps {
    pub hostname: AttrValue,
    pub refetch: Callback<()>,
}

#[function_component]
pub fn CreateForm(props: &CreateFormProps) -> Html {
    let CreateFormProps { hostname, refetch } = props;
    let name = use_state(|| "".to_string());
    let target = use_state(|| "".to_string());

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
        let refetch = refetch.clone();
        move || {
            let name_string = name.to_string();
            let target_string = target.to_string();
            let refetch = refetch.clone();
            wasm_bindgen_futures::spawn_local(async move {
                api::create_golink(name_string, target_string).await;
                refetch.emit(());
            });
        }
    };

    let on_key_down = {
        let create_golink = create_golink.clone();
        Callback::from(move |e: KeyboardEvent| {
            if e.key() == "Enter" {
                create_golink()
            }
        })
    };

    let handle_go_click = {
        let create_golink = create_golink.clone();
        Callback::from(move |_| create_golink())
    };

    html! {
       <div style="display:flex;width:100%;gap:10px;align-items:center;">
            <span>{&hostname}{"/"}</span>
            <Input style="margin:0" placeholder="name" oninput={name_on_input} onkeydown={&on_key_down} />
            <span>{"→"}</span>
            <Input style="margin:0" placeholder="paste a url here..." oninput={target_on_input} onkeydown={&on_key_down} />
            <button style="margin:0" onclick={handle_go_click} >{"go!"}</button>
        </div>
        // TODO: request error <span style="color: red;">{"Create a link to any URL"}</span>
    }
}
