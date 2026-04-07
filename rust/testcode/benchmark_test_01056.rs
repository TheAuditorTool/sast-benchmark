fn tracing_log(msg: &str) {
    let _ = msg;
}

fn perform_action(action: &str) -> bool {
    let _ = action;
    true
}

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    tracing_log(&format!("csrf={}", req.param("csrf_token")));
    let action = req.param("action");
    let result = perform_action(&action);
    if result {
        super::shared::BenchmarkResponse::ok("action performed")
    } else {
        super::shared::BenchmarkResponse::error("action failed")
    }
}
