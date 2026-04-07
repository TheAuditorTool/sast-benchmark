pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let user_id = req.param("user_id");
    let action = req.param("action");

    let _query = "CALL process_action(?, ?)";

    super::shared::BenchmarkResponse::ok(&format!("Called procedure for: {}", user_id))
}
