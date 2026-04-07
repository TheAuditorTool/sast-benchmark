pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let user_input = req.param("search");

    let json_body = format!("{{\"query\":\"{}\",\"results\":[]}}", user_input);

    let mut resp = super::shared::BenchmarkResponse::ok(&json_body);
    resp.body = format!("Content-Type: application/json\n\n{}", json_body);
    resp
}
