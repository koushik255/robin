use robin::{CmdRequest, CmdResponse};
use std::process::exit;

#[tokio::main]
async fn main() {
    // Everything after the binary name: e.g. `ls -la` → ["ls", "-la"]
    let mut args = std::env::args().skip(1);

    let command = match args.next() {
        Some(c) => c,
        None => {
            eprintln!("usage: cargo run --bin client -- <command> [args...]");
            exit(2);
        }
    };
    let rest: Vec<String> = args.collect();

    // Point this at your server. Localhost for now.
    let url = "http://kouskous:3000/run";

    let resp: CmdResponse = reqwest::Client::new()
        .post(url)
        .json(&CmdRequest {
            command,
            args: rest,
        })
        .send()
        .await
        .expect("request failed — is the daemon running?")
        .json()
        .await
        .expect("failed to parse daemon response");

    // Mirror the remote output locally.
    print!("{}", resp.stdout);
    eprint!("{}", resp.stderr);

    exit(resp.exit_code.unwrap_or(1));
}
