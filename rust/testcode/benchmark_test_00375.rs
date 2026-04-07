struct GreetTemplate<'a> {
    name: &'a str,
}

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let name = req.param("name");

    let tmpl = GreetTemplate { name: &name };
    let html = render_template(&tmpl);

    super::shared::BenchmarkResponse::ok(&html)
}

fn render_template(tmpl: &GreetTemplate) -> String {
    let escaped = tmpl.name
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;");
    format!("<p>Hello, {}!</p>", escaped)
}
