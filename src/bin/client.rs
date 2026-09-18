use robin::DirListing;

#[tokio::main]
async fn main() {
    let listing: DirListing = reqwest::Client::new()
        .get("http://100.98.83.82:3006/ls")
        .send()
        .await
        .expect("request to daemon failed — is it running?")
        .json()
        .await
        .expect("bad response from daemon");

    println!("{}", serde_json::to_string_pretty(&listing).unwrap());
}
