pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let preview_url = req.param("preview_url");

    let html = format!(
        "<html><body><iframe src='{}' width='600' height='400'></iframe></body></html>",
        preview_url
    );

    super::shared::BenchmarkResponse::ok(&html)
}
