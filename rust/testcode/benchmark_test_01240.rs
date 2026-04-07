use serde::Deserialize;

#[derive(Deserialize)]
struct Command {
    action: String,
    payload: serde_json::Value,
    exec: Option<String>,
}

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let body = req.body_str();
    let cmd: Command = match serde_json::from_str(&body) {
        Ok(c) => c,
        Err(e) => return super::shared::BenchmarkResponse::bad_request(&e.to_string()),
    };
    super::shared::BenchmarkResponse::ok(&format!("Action: {} exec: {:?}", cmd.action, cmd.exec))
}
