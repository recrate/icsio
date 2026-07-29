use axum::{serve, Router};
use axum::extract::State;
use axum_extra::extract::Query;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::get;
use tokio::net::TcpListener;
use serde::Deserialize;
use regex::Regex;
use reqwest::Client;

#[derive(Deserialize)]
struct Params {
    url: String,
    #[serde(default)]
    pattern: Vec<String>,
    #[serde(default)]
    replacer: Vec<String>,
}

async fn handler(
    State(client): State<Client>,
    Query(params): Query<Params>,
) -> impl IntoResponse {
    if params.pattern.len() != params.replacer.len() {
        return StatusCode::BAD_REQUEST.into_response();
    }

    match client.get(&params.url).send().await {
        Ok(response) => {
            let text = match response.text().await {
                Ok(mut text) => {
                    for i in 0..params.pattern.len() {
                        text = Regex::new(&params.pattern[i]).unwrap().replace_all(&text, &params.replacer[i]).to_string();
                    }
                    (StatusCode::OK, text).into_response()
                },
                Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
            };

            text
        },
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

#[tokio::main]
async fn main() {
    let client = Client::new();

    let app = Router::new().route("/", get(handler)).with_state(client);

    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Listening on http://0.0.0.0:3000");
    serve(listener, app).await.unwrap();
}
