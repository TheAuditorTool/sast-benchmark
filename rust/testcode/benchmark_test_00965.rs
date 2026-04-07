pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let target = req.param("target");
    let _parsed = parse_url(&target);
    let redirect = format!("Location: {}", target);
    super::shared::BenchmarkResponse::ok(&redirect)
}
fn parse_url(url: &str) -> Option<String> {
    if url.contains("://") || url.starts_with("/") { Some(url.to_string()) } else { None }
}
