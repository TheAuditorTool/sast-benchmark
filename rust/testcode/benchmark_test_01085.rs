pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let url = req.param("url");
    if url.starts_with("/") {
        super::shared::BenchmarkResponse::ok(&format!("Location: {}", url))
    } else {
        super::shared::BenchmarkResponse::bad_request("Must be relative URL")
    }
}
