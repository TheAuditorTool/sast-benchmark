fn html_escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#x27;")
}

fn sanitize(s: &str) -> String {
    html_escape(s)
}

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let user_input = req.param("name");

    let html = format!("<div>{}</div>", sanitize(&user_input));

    super::shared::BenchmarkResponse::ok(&html)
}
