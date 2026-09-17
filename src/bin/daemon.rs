use axum::{Json, Router, routing::post};
use robin::{CmdRequest, CmdResponse};
use std::process::Command;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/run", post(run_command));

    let addr = "0.0.0.0:3000";
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    println!("daemon listening on http://{addr}");

    axum::serve(listener, app).await.unwrap();
}

async fn run_command(Json(req): Json<CmdRequest>) -> Json<CmdResponse> {
    // std::process::Command is blocking, so run it off the async worker thread.
    let output =
        tokio::task::spawn_blocking(move || Command::new(&req.command).args(&req.args).output())
            .await
            .expect("spawn_blocking panicked");

    let response = match output {
        Ok(out) => CmdResponse {
            stdout: String::from_utf8_lossy(&out.stdout).into_owned(),
            stderr: String::from_utf8_lossy(&out.stderr).into_owned(),
            exit_code: out.status.code(),
        },
        Err(e) => CmdResponse {
            stdout: String::new(),
            stderr: format!("failed to run command: {e}"),
            exit_code: None,
        },
    };

    Json(response)
}
