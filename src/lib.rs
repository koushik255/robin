use serde::{Deserialize, Serialize};

// client -> daemon
#[derive(Debug, Serialize, Deserialize)]
pub struct CmdRequest {
    pub command: String,
    pub args: Vec<String>,
}

//bkac - > client
#[derive(Debug, Serialize, Deserialize)]
pub struct CmdResponse {
    pub stdout: String,
    pub stderr: String,
    pub exit_code: Option<i32>,
}
