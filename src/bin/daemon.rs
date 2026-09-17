use axum::{Json, Router, extract::State, routing::post};
use robin::{CmdRequest, CmdResponse};
use std::path::PathBuf;
use std::process::Command;
use std::sync::Arc;
use std::sync::Mutex;
use std::time::Instant;

type SharedCwd = Arc<Mutex<PathBuf>>;

#[tokio::main]
async fn main() {
    let start_dir = std::env::current_dir().expect("Count not read current dir");
    let cwd: SharedCwd = Arc::new(Mutex::new(start_dir));

    let app = Router::new()
        .route("/run", post(run_command))
        .with_state(cwd);

    let addr = "100.98.83.82:3006";

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    println!("daemon listening on http://{addr}");

    axum::serve(listener, app).await.unwrap();
}

async fn run_command(
    State(cwd): State<SharedCwd>,
    Json(req): Json<CmdRequest>,
) -> Json<CmdResponse> {
    // std::process::Command is blocking, so run it off the async worker thread.
    //
    //
    // is thier a settings for automating improts for rustlsp?
    let start = Instant::now();

    println!("{:?}", req);
    if req.command == "cd" {
        if req.args == [".."] {
            *cwd.lock().unwrap() = cwd.lock().unwrap().parent().unwrap().to_path_buf();
            return Json(CmdResponse {
                stdout: String::new(),
                stderr: String::new(),
                exit_code: Some(0),
            });
        }
        return Json(change_dir(&cwd, &req.args));
    }

    // snapshot current dir realse before await
    let dir = { cwd.lock().unwrap().clone() };
    let output = tokio::task::spawn_blocking(move || {
        Command::new(&req.command)
            .args(&req.args)
            .current_dir(&dir)
            .output()
    })
    .await
    .expect("spawn_blocking panicked");

    let duration = start.elapsed();
    println!("command took {}ms", duration.as_millis());

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
fn change_dir(cwd: &SharedCwd, args: &[String]) -> CmdResponse {
    let Some(target) = args.first() else {
        return CmdResponse {
            stdout: String::new(),
            stderr: "cd: missing path argument\n".into(),
            exit_code: Some(1),
        };
    };

    let mut guard = cwd.lock().unwrap();

    // Relative paths resolve against the current remembered dir.
    let candidate = {
        let p = PathBuf::from(target);
        if p.is_absolute() { p } else { guard.join(p) }
    };

    // canonicalize() also fails if the path doesn't exist, covering that case.
    if candidate.is_dir() {
        *guard = candidate.clone();
        CmdResponse {
            stdout: format!("{}\n", candidate.display()),
            stderr: String::new(),
            exit_code: Some(0),
        }
    } else {
        CmdResponse {
            stdout: String::new(),
            stderr: format!("cd: not a directory: {target}\n"),
            exit_code: Some(1),
        }
    }
}
