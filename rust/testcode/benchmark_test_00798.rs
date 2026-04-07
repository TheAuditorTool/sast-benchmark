pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let input = req.param("val");

    let filtered = input.replace('<', "").replace('>', "");
    let html = format!("<input value=\"{}\">", filtered);

    super::shared::BenchmarkResponse::ok(&html)
}
