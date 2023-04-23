use crate::api;
use crate::components::{button::DoubleClickButton, input::Input};
use crate::GoLink;
use stylist::yew::use_style;
use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct GoLinksTableRowProps {
    golink: GoLink,
    refetch: Callback<()>,
}

#[function_component]
pub fn GoLinksTableRow(props: &GoLinksTableRowProps) -> Html {
    let GoLinksTableRowProps { golink, refetch } = props;
    let editing = use_state(|| false);
    let new_target = use_state(|| golink.target.clone());

    let show_editing = {
        let editing = editing.clone();
        Callback::from(move |_| {
            editing.set(true);
        })
    };

    let new_target_on_input = {
        let new_target = new_target.clone();
        Callback::from(move |value| {
            new_target.set(value);
        })
    };

    let handle_save_click = {
        let editing = editing.clone();
        let refetch = refetch.clone();
        let name = golink.name.clone();
        let new_target = new_target.clone();
        Callback::from(move |_| {
            let editing = editing.clone();
            let refetch = refetch.clone();
            let name = name.clone();
            let new_target = new_target.clone();
            wasm_bindgen_futures::spawn_local(async move {
                api::edit_golink(&name, &(*new_target)).await.unwrap();
                editing.set(false);
                refetch.emit(());
            });
        })
    };

    let on_edit_key_down = {
        let edit_golink = handle_save_click.clone();
        Callback::from(move |e: KeyboardEvent| {
            if e.key() == "Enter" {
                edit_golink.emit(MouseEvent::new("dummy").unwrap());
            }
        })
    };

    let handle_delete_click = {
        let refetch = refetch.clone();
        let name = golink.name.clone();
        Callback::from(move |_| {
            let refetch = refetch.clone();
            let name = name.clone();
            wasm_bindgen_futures::spawn_local(async move {
                api::delete_golink(&name).await.unwrap();
                refetch.emit(());
            });
        })
    };

    let button_container_style = use_style!(
        r#"
        display: flex;
        gap: 10px;
        width: 100px;
        "#
    );

    html! {
        <tr>
            <td>{&golink.name}</td>
            <td>
            {
                if *editing {
                    html! {
                        <Input value={(*new_target).clone()} oninput={new_target_on_input} onkeydown={on_edit_key_down} />
                    }
                } else {
                    html! {
                        {&golink.target}
                    }
                }
            }
           </td>
            <td>{&golink.visits}</td>
            <td>
                <div class={button_container_style}>
                    <DoubleClickButton
                        first_label="Edit"
                        second_label="Save"
                        on_first_click={show_editing}
                        on_second_click={handle_save_click} />
                    <DoubleClickButton
                        first_label="Delete"
                        second_label="Confirm?"
                        on_second_click={handle_delete_click} />
                </div>
            </td>
        </tr>
    }
}

#[derive(PartialEq, Properties)]
pub struct GoLinksTableProps {
    pub golinks: Vec<GoLink>,
    pub refetch: Callback<()>,
}

#[function_component]
pub fn GoLinksTable(props: &GoLinksTableProps) -> Html {
    let GoLinksTableProps { golinks, refetch } = props;

    let table_container_style = use_style!(
        r#"
        width: 100%;
        max-height: 70vh;
        overflow-y: auto;
        "#
    );

    let th_style = use_style!(
        r#"
        position: sticky;
        top: 0px;
        background: white;
        "#
    );

    html! {
        <div class={table_container_style}>
            <table>
                <thead>
                    <tr>
                        {
                        vec!["Name", "URL", "Total Visits", "Actions"].iter().map(|col| html! {

                            <th class={th_style.clone()}>{col}</th>
                    }).collect::<Html>()}
                    </tr>
                </thead>
                <tbody>
                    {
                        golinks.iter().map(|golink| html! {
                        <GoLinksTableRow golink={golink.clone()} refetch={refetch.clone()} />
                    }).collect::<Html>()}
                </tbody>
            </table>
        </div>
    }
}
