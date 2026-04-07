pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let _url = req.param("url");
    let location = if 3 * 3 == 9 {
        "Location: /dashboard"
    } else {
        "Location: /untrusted"
    };
    super::shared::BenchmarkResponse::ok(location)
}
