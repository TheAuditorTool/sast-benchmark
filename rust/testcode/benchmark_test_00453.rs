pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let url = req.param("url");

    let html = format!(
        "<html><body><a href='{}'>Click here</a></body></html>",
        url
    );

    super::shared::BenchmarkResponse::ok(&html)
}
