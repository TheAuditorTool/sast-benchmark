pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let username = req.param("username");

    let html = format!("<script>var user = \"{}\";</script>", username);

    super::shared::BenchmarkResponse::ok(&html)
}
