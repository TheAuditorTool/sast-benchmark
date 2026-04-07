pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let name = req.param("name");
    let encoded = name
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#x27;");
    let html = format!("<h1>Hello, {}!</h1>", encoded);
    super::shared::BenchmarkResponse::ok(&html)
}
