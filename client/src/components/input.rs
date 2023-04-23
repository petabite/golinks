use wasm_bindgen::JsCast;
use web_sys::{EventTarget, HtmlInputElement};
use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct InputProps {
    pub value: AttrValue,
    pub style: AttrValue,
    pub oninput: Callback<String>,
    pub onkeydown: Callback<KeyboardEvent>,
    pub placeholder: AttrValue,
}

#[function_component]
pub fn Input(props: &InputProps) -> Html {
    let InputProps {
        style,
        value,
        placeholder,
        oninput,
        onkeydown,
    } = props;

    let handle_on_input: Callback<InputEvent> = {
        let oninput = oninput.clone();
        Callback::from(move |e: InputEvent| {
            let target: Option<EventTarget> = e.target();
            let input = target.and_then(|t| t.dyn_into::<HtmlInputElement>().ok());

            if let Some(input) = input {
                oninput.emit(input.value());
            }
        })
    };

    html! {
        <input style={style} type="text" value={value} placeholder={placeholder} oninput={handle_on_input} onkeydown={onkeydown} />
    }
}
