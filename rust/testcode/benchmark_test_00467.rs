pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let target = req.param("url");
    if target.contains("example.com") {
        super::shared::BenchmarkResponse::ok(&format!("Location: {}", target))
    } else {
        super::shared::BenchmarkResponse::bad_request("Invalid redirect")
    }
}
