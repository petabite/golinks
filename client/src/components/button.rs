use stylist::yew::use_style;
use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct DoubleClickButtonProps {
    pub first_label: String,
    pub second_label: String,
    #[prop_or_default]
    pub on_first_click: Callback<MouseEvent>,
    #[prop_or_default]
    pub on_second_click: Callback<MouseEvent>,
}

#[function_component]
pub fn DoubleClickButton(props: &DoubleClickButtonProps) -> Html {
    let DoubleClickButtonProps {
        first_label,
        second_label,
        on_first_click,
        on_second_click,
    } = props;
    let first_clicked = use_state(|| false);

    let handle_click = {
        let first_clicked = first_clicked.clone();
        let on_first_click = on_first_click.clone();
        let on_second_click = on_second_click.clone();
        Callback::from(move |e| {
            let first_clicked = first_clicked.clone();
            let on_first_click = on_first_click.clone();
            let on_second_click = on_second_click.clone();
            wasm_bindgen_futures::spawn_local(async move {
                if *first_clicked {
                    first_clicked.set(false);
                    on_second_click.emit(e);
                } else {
                    first_clicked.set(true);
                    on_first_click.emit(e);
                }
            });
        })
    };

    let button_style = use_style!(
        r#"
        padding: 0;
        margin: 0;
        "#
    );

    html! {
        <button class={classes!("button", "button-clear", button_style)} onclick={handle_click}>{if *first_clicked {second_label} else {first_label}}</button>
    }
}
