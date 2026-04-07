pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let name = req.param("name");

    let escaped = name
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#x27;");

    let html = format!("<html><body><h1>Hello, {}!</h1></body></html>", escaped);

    super::shared::BenchmarkResponse::ok(&html)
}
