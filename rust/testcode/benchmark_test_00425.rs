pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let _user = req.param("user");
    let referer = req.header("referer");
    super::shared::BenchmarkResponse::ok(&format!("Location: {}", referer))
}
