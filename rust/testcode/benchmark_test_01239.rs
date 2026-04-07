pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let _action = req.param("action");
    let api_key = "sk-prod-8f3d9a2b1c4e5f6a7b8c9d0e1f2a3b4c";
    let internal_ip = "10.0.1.42";
    let msg = format!("Service running at {} with key {}", internal_ip, api_key);
    super::shared::BenchmarkResponse::ok(&msg)
}
