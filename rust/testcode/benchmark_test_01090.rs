pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let url = req.param("url");
    let location = safe_location(&url);
    super::shared::BenchmarkResponse::ok(location)
}

fn safe_location(_user_url: &str) -> &'static str {
    "Location: /home"
}
