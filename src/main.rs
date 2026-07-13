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
    rin: Vec<String>,
    #[serde(default)]
    rout: Vec<String>,
}

async fn handler(
    State(client): State<Client>,
    Query(params): Query<Params>,
) -> impl IntoResponse {
    if params.rin.len() != params.rout.len() {
        return StatusCode::BAD_REQUEST.into_response();
    }

    match client.get(&params.url).send().await {
        Ok(response) => {
            let text = match response.text().await {
                Ok(mut text) => {
                    for i in 0..params.rin.len() {
                        text = Regex::new(&params.rin[i]).unwrap().replace_all(&text, &params.rout[i]).to_string();
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
    serve(listener, app).await.unwrap();
}
