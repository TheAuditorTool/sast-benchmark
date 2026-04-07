pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let _url = req.param("url");
    let location = "Location: /dashboard";
    super::shared::BenchmarkResponse::ok(location)
}
