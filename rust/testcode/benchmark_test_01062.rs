fn html_escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#x27;")
}

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let name = req.param("name");

    let mut m = std::collections::HashMap::new();
    m.insert("raw", name.clone());
    m.insert("safe", html_escape(&name));

    let val = m.get("safe").unwrap();
    let html = format!("<p>{}</p>", val);

    super::shared::BenchmarkResponse::ok(&html)
}
