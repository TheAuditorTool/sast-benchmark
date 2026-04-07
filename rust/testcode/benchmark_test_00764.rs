pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let content = req.param("content");

    let html = format!("<iframe src=\"data:text/html,{}\"></iframe>", content);

    super::shared::BenchmarkResponse::ok(&html)
}
