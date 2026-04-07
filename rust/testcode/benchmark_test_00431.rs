pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let dest = req.cookie("redirect_after");
    super::shared::BenchmarkResponse::ok(&format!("Location: {}", dest))
}
