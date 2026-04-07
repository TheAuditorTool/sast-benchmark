pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let _action = req.param("action");
    let target = "/dashboard";
    super::shared::BenchmarkResponse::ok(&format!("Location: {}", target))
}
