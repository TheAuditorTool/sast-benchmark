pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let label = req.param("label");

    let html = format!("<svg><text x='10' y='20'>{}</text></svg>", label);

    super::shared::BenchmarkResponse::ok(&html)
}
