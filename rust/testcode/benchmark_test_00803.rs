pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let url = req.param("url");
    if url.starts_with("https://") {
        let location = format!("Location: {}", url);
        super::shared::BenchmarkResponse::ok(&location)
    } else {
        super::shared::BenchmarkResponse::bad_request("HTTPS only")
    }
}
