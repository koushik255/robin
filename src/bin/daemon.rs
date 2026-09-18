use axum::{Json, Router, routing::get};
use robin::{DirListing, FileEntry};
use std::fs;

async fn list_dir() -> Json<DirListing> {
    let cwd = std::env::current_dir().expect("cannot read current dir");

    // wonder how much time these cost us?
    let entries: Vec<FileEntry> = fs::read_dir(&cwd)
        .expect("cannot read directory")
        .filter_map(|entry| entry.ok()) // skip any entries we failed to read
        .map(|entry| {
            let name = entry.file_name().to_string_lossy().into_owned();
            let is_dir = match entry.file_type() {
                Ok(ft) => ft.is_dir(),
                Err(_) => false,
            };
            FileEntry { name, is_dir }
        })
        .collect();

    Json(DirListing {
        path: cwd.to_string_lossy().into_owned(),
        entries,
    })
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/ls", get(list_dir));

    let addr = "100.98.83.82:3006";
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    println!("daemon listening on http://{addr}");

    axum::serve(listener, app).await.unwrap();
}
