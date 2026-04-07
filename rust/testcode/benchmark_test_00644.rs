pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let user_data = req.param("data");

    let json_val = format!("\"{}\"", user_data);
    let html = format!("<script>var config = {};</script>", json_val);

    super::shared::BenchmarkResponse::ok(&html)
}
