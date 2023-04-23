use crate::api;
use crate::GoLink;
use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct GoLinksTableProps {
    pub golinks: Vec<GoLink>,
    pub refetch: Callback<()>,
}

#[function_component]
pub fn GoLinksTable(props: &GoLinksTableProps) -> Html {
    let GoLinksTableProps { golinks, refetch } = props;

    let handle_delete_click = |name: &String| -> yew::Callback<web_sys::MouseEvent> {
        let name = name.clone();
        let refetch = refetch.clone();
        Callback::from(move |_| {
            let name_string = name.to_string();
            let refetch = refetch.clone();
            wasm_bindgen_futures::spawn_local(async move {
                api::delete_golink(&name_string).await;
                refetch.emit(());
            });
        })
    };
    html! {
        <>
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
        </>
    }
}
