pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let action = req.param("action");

    let html = format!("<a href=\"javascript:{}\">Click</a>", action);

    super::shared::BenchmarkResponse::ok(&html)
}
