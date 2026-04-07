pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let age = req.param("age");
    let result = format!("Processing age: {}", age);
    super::shared::BenchmarkResponse::ok(&result)
}
