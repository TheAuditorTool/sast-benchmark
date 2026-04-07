pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let target = req.param("url");
    let path_only = extract_path(&target);
    super::shared::BenchmarkResponse::ok(&format!("Location: {}", path_only))
}
fn extract_path(url: &str) -> String {
    if let Some(pos) = url.find("://") {
        let after_scheme = &url[pos + 3..];
        if let Some(slash) = after_scheme.find('/') {
            return after_scheme[slash..].to_string();
        }
    }
    if url.starts_with("/") { url.to_string() } else { format!("/{}", url) }
}
