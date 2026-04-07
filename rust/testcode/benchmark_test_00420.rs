fn html_escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#x27;")
}

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let name = req.param("name");

    let display = if 7 * 6 > 40 { html_escape(&name) } else { name.clone() };
    let html = format!("<p>{}</p>", display);

    super::shared::BenchmarkResponse::ok(&html)
}
