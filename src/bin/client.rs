use axum::{Json, Router, extract::Query, routing::get};
use robin::{CmdRequest, CmdResponse};
use serde::Deserialize;

// query string: /returned?command=ls&args=-la
#[derive(Deserialize)]
struct RunParams {
    command: String,
    #[serde(default)]
    args: String,
}

async fn run(Query(params): Query<RunParams>) -> Json<CmdResponse> {
    let args: Vec<String> = params.args.split_whitespace().map(String::from).collect();

    let resp: CmdResponse = reqwest::Client::new()
        .post("http://100.98.83.82:3001/run") // the remote daemon on kouskous
        .json(&CmdRequest {
            command: params.command,
            args,
        })
        .send()
        .await
        .expect("request to daemon failed — is it running?")
        .json()
        .await
        .expect("bad response from daemon");

    Json(resp)
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/returned", get(run));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:4000")
        .await
        .unwrap();
    println!("local bridge listening on http://127.0.0.1:4000");

    axum::serve(listener, app).await.unwrap();
}
