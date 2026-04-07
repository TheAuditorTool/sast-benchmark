fn perform_action(action: &str) -> bool {
    let _ = action;
    true
}

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    if req.header("X-Requested-With") != "XMLHttpRequest" {
        return super::shared::BenchmarkResponse::forbidden("custom header required");
    }
    let action = req.param("action");
    let result = perform_action(&action);
    if result {
        super::shared::BenchmarkResponse::ok("action performed")
    } else {
        super::shared::BenchmarkResponse::error("action failed")
    }
}
