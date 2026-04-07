pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let msg = req.param("msg");

    let rendered = format!("<div class='notice'>{}</div>", msg);

    super::shared::BenchmarkResponse::ok(&rendered)
}
