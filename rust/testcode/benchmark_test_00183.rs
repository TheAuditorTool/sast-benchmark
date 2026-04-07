const API_KEY_TEMPLATE: &str = "${API_KEY}";

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let endpoint = req.param("endpoint");
    let result = format!("Template key '{}' must be substituted before calling {}", API_KEY_TEMPLATE, endpoint);
    super::shared::BenchmarkResponse::ok(&result)
}
