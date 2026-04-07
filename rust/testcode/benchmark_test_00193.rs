pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let _action = req.param("action");
    let request_id = "req-a1b2c3d4";
    let response = format!(r#"{{"error":"internal","request_id":"{}"}}"#, request_id);
    super::shared::BenchmarkResponse::error(&response)
}
