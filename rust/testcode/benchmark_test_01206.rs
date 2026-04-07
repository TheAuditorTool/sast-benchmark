pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let service = req.param("service");
    let endpoint = match service.as_str() {
        "payments" => "https://payments.internal.corp/health",
        "inventory" => "https://inventory.internal.corp/health",
        "shipping" => "https://shipping.internal.corp/health",
        _ => return super::shared::BenchmarkResponse::bad_request("Unknown service"),
    };
    super::shared::BenchmarkResponse::ok(&format!("Health check: {}", endpoint))
}
