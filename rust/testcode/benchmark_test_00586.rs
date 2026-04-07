fn html_escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#x27;")
}

struct PageTemplate<'a> {
    title: &'a str,
}

fn render_template(t: &PageTemplate) -> String {
    let escaped = html_escape(t.title);
    format!("<title>{}</title>", escaped)
}

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let title = req.param("title");

    let tmpl = PageTemplate { title: &title };
    let html = render_template(&tmpl);

    super::shared::BenchmarkResponse::ok(&html)
}
