pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let user_url = req.param("avatar_url");

    let html = format!("<html><body><img src='{}' alt='avatar'></body></html>", user_url);

    super::shared::BenchmarkResponse::ok(&html)
}
