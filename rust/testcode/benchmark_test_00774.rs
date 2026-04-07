pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let url = req.param("url");
    let host = extract_host(&url);
    if is_allowed_host(&host) {
        let location = format!("Location: {}", url);
        super::shared::BenchmarkResponse::ok(&location)
    } else {
        super::shared::BenchmarkResponse::bad_request("Disallowed host")
    }
}

fn extract_host(url: &str) -> String {
    url.split("://").nth(1)
        .and_then(|s| s.split('/').next())
        .unwrap_or("")
        .to_string()
}

fn is_allowed_host(host: &str) -> bool {
    matches!(host, "example.com" | "api.example.com" | "cdn.example.com")
}
