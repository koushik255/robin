use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct FileEntry {
    pub name: String,
    pub is_dir: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DirListing {
    pub path: String,
    pub entries: Vec<FileEntry>,
}
