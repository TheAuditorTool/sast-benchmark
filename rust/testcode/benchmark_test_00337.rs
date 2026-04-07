fn admin_action() -> String {
    "admin_action_performed".to_string()
}

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    if req.header("X-Role") == "admin" {
        let result = admin_action();
        return super::shared::BenchmarkResponse::ok(&result);
    }
    super::shared::BenchmarkResponse::forbidden("access denied")
}
