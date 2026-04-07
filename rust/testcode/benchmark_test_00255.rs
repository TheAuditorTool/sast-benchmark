pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let url = req.param("url");
    if is_trusted_host(&url) {
        let location = format!("Location: {}", url);
        super::shared::BenchmarkResponse::ok(&location)
    } else {
        super::shared::BenchmarkResponse::bad_request("Untrusted host")
    }
}

fn is_trusted_host(url: &str) -> bool {
    url.starts_with("https://example.com/") || url.starts_with("https://app.example.com/")
}
