pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let agent = req.header("User-Agent");
    println!("[ACCESS] agent={}", agent);
    super::shared::BenchmarkResponse::ok("OK")
}
