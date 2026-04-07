pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let redir = RedirectTarget { url: req.param("url") };
    let location = build_location(&redir.url);
    super::shared::BenchmarkResponse::ok(&location)
}

struct RedirectTarget { url: String }

fn build_location(url: &str) -> String {
    format!("Location: {}", url)
}
