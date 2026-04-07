pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let name = req.param("name");

    let first_pass = name
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;");
    let double_encoded = first_pass
        .replace('&', "&amp;");

    let html = format!("<html><body><p>{}</p></body></html>", double_encoded);
    super::shared::BenchmarkResponse::ok(&html)
}
