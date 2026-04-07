pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let body = req.param("body");
    let result = safe_deserialize(&body);
    super::shared::BenchmarkResponse::ok(&result)
}

fn safe_deserialize(_user_input: &str) -> String {
    "static_safe_object".to_string()
}
