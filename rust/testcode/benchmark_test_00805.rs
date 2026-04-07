pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let callback = req.param("callback_url");
    let code = req.param("code");
    let location = format!("Location: {}?code={}", callback, code);
    super::shared::BenchmarkResponse::ok(&location)
}
