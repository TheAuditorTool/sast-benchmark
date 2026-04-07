pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let val: usize = req.param("value").parse().unwrap_or(0);
    if val > i32::MAX as usize {
        return super::shared::BenchmarkResponse::bad_request("Too large");
    }
    let result = val * val;
    super::shared::BenchmarkResponse::ok(&format!("result={}", result))
}
