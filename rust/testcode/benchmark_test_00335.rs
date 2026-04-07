pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let user_name = req.param("name");

    let html = format!("<html><script>var name='{}';</script><body>Hi</body></html>", user_name);

    super::shared::BenchmarkResponse::ok(&html)
}
