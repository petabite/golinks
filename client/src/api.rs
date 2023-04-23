use gloo_net::http::{Method, Request};
use serde_json::json;

use crate::GoLink;

static API_URL: &str = {
    match option_env!("DEV") {
        Some(_) => "http://localhost:7890/link",
        None => "/link",
    }
};

async fn make_request(method: Method, url: &str, body: Option<String>) -> Result<String, String> {
    let mut request = Request::new(url).method(method);
    if let Some(body) = body {
        request = request.body(&body);
    }
    let response = request.send().await.unwrap();

    match response.ok() {
        true => Ok(response.text().await.unwrap()),
        false => Err(response.text().await.unwrap()),
    }
}

pub async fn get_all_golinks() -> Result<Vec<GoLink>, String> {
    let response = make_request(Method::GET, API_URL, None).await;

    match response {
        Ok(response) => Ok(serde_json::from_str(&response).unwrap()),
        Err(err) => Err(err),
    }
}

pub async fn create_golink(name: String, target: String) -> Result<GoLink, String> {
    let body = json!({
        "name": name,
        "target": target,
    });
    let response = make_request(Method::POST, API_URL, Some(body.to_string())).await;
    match response {
        Ok(response) => Ok(serde_json::from_str(&response).unwrap()),
        Err(err) => Err(err),
    }
}

pub async fn edit_golink(name: &str, target: &str) -> Result<GoLink, String> {
    let body = json!({
        "name": name,
        "target": target,
    });
    let response = make_request(
        Method::PUT,
        format!("{}/{}", API_URL, name).as_str(),
        Some(body.to_string()),
    )
    .await;
    match response {
        Ok(response) => Ok(serde_json::from_str(&response).unwrap()),
        Err(err) => Err(err),
    }
}

pub async fn delete_golink(name: &str) -> Result<String, String> {
    make_request(
        Method::DELETE,
        format!("{}/{}", API_URL, name).as_str(),
        None,
    )
    .await
}
