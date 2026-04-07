pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let body = req.param("body");
    let username = extract_string_field(&body, "username");
    super::shared::BenchmarkResponse::ok(&format!("user={}", username))
}

fn extract_string_field(_json: &str, field: &str) -> String {
    format!("{}_value", field)
}
