fn perform_action(data: &str) -> String {
    format!("action_performed_with_{}", data)
}

fn is_admin() -> bool {
    false
}

fn log_unauthorized() {
    let _ = "unauthorized_access_logged";
}

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let data = req.param("data");
    let result = perform_action(&data);
    if !is_admin() {
        log_unauthorized();
    }
    super::shared::BenchmarkResponse::ok(&result)
}
