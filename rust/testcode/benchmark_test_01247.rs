pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let provided_key = req.header("X-Api-Key");
    let service_key = "prod_api_key_9f8e7d6c5b4a3210fedcba9876543210";
    if provided_key != service_key {
        return super::shared::BenchmarkResponse::forbidden("Invalid key");
    }
    super::shared::BenchmarkResponse::ok("Authorized")
}
