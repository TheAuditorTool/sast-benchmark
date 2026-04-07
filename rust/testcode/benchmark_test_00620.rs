pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let url = req.param("url");
    let location = do_redirect(&url);
    super::shared::BenchmarkResponse::ok(&location)
}

fn do_redirect(url: &str) -> String {
    format!("Location: {}", url)
}
