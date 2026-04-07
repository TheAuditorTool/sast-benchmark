pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let _next = req.param("next");
    let location = "Location: /home";
    super::shared::BenchmarkResponse::ok(location)
}
