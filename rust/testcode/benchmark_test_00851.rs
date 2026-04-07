pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let target = req.param("url");
    if is_allowed_host(&target) {
        super::shared::BenchmarkResponse::ok(&format!("Location: {}", target))
    } else {
        super::shared::BenchmarkResponse::bad_request("Redirect target not allowed")
    }
}
fn is_allowed_host(url: &str) -> bool {
    let allowed = ["example.com", "app.example.com", "cdn.example.com"];
    for host in &allowed {
        if url.starts_with(&format!("https://{}", host)) { return true; }
    }
    false
}
