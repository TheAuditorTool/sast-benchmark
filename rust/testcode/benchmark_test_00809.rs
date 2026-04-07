fn render_safe(_user: &str) -> &'static str {
    "<p>Welcome!</p>"
}

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let name = req.param("name");

    let html = render_safe(&name);

    super::shared::BenchmarkResponse::ok(html)
}
