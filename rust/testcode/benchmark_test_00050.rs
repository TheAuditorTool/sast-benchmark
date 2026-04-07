pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let name = req.param("name");

    let auto_escaped = name
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#x27;");

    let html = format!("<html><body><p>Welcome, {}!</p></body></html>", auto_escaped);
    super::shared::BenchmarkResponse::ok(&html)
}
