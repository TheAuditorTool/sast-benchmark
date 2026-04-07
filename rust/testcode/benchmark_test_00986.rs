pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let url = req.param("url");

    let html = format!("<meta http-equiv=\"refresh\" content=\"0;url={}\">", url);

    super::shared::BenchmarkResponse::ok(&html)
}
