use gloo_net::http::Request;
use serde_json::json;

use crate::GoLink;

static API_URL: &str = {
    match option_env!("DEV") {
        Some(_) => "http://localhost:7890/link",
        None => "/link",
    }
};

// TODO: golinks table filtering
// TODO: golinks table created at
pub async fn get_all_golinks() -> Vec<GoLink> {
    Request::get(&API_URL)
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap()
}

pub async fn create_golink(name: String, target: String) -> GoLink {
    let body = json!({
        "name": name,
        "target": target,
    });
    Request::post(&API_URL)
        .body(&body.to_string())
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap()
}

// TODO: implement edit golink
pub async fn edit_golink(name: &str, target: &str) -> GoLink {
    let body = json!({
        "name": name,
        "target": target,
    });
    Request::put(format!("{}/{}", &API_URL, &name).as_str())
        .body(&body.to_string())
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap()
}

pub async fn delete_golink(name: &str) {
    Request::delete(format!("{}/{}", &API_URL, &name).as_str())
        .send()
        .await
        .unwrap();
}
