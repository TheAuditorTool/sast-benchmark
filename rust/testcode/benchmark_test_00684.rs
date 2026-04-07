pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let user_input = req.param("msg");

    let html = render_safe_template(&user_input);

    super::shared::BenchmarkResponse::ok(&html)
}

fn render_safe_template(message: &str) -> String {
    format!("<p>{}</p>", message)
}
