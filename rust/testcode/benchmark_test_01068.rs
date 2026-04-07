pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let url = req.param("url");
    if url.contains("example.com") {
        let location = format!("Location: {}", url);
        super::shared::BenchmarkResponse::ok(&location)
    } else {
        super::shared::BenchmarkResponse::bad_request("Invalid redirect")
    }
}
