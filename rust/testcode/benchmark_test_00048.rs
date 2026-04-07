fn html_escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#x27;")
}

struct Profile {
    name: String,
}

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let profile = Profile {
        name: html_escape(&req.param("name")),
    };

    let html = format!("<h2>{}</h2>", profile.name);

    super::shared::BenchmarkResponse::ok(&html)
}
