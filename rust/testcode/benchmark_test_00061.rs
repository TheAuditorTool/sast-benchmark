pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let url = req.param("url");
    if is_safe_relative_path(&url) {
        let location = format!("Location: {}", url);
        super::shared::BenchmarkResponse::ok(&location)
    } else {
        super::shared::BenchmarkResponse::bad_request("Invalid path")
    }
}

fn is_safe_relative_path(url: &str) -> bool {
    !url.starts_with("//") &&
    !url.contains("://") &&
    url.starts_with('/') &&
    url.chars().all(|c| c.is_ascii_alphanumeric() || "/._-".contains(c))
}
