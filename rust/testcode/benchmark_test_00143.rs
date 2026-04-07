pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let color = req.param("color");

    let html = format!("<div style=\"color: {}\">text</div>", color);

    super::shared::BenchmarkResponse::ok(&html)
}
