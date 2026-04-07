pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let referer = req.header("Referer");
    let location = format!("Location: {}", referer);
    super::shared::BenchmarkResponse::ok(&location)
}
