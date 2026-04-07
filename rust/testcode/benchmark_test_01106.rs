pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let body = req.param("body");
    let variant = deserialize_enum_variant(&body);
    super::shared::BenchmarkResponse::ok(&format!("variant={}", variant))
}

fn deserialize_enum_variant(_json: &str) -> String {
    "ArbitraryVariant".to_string()
}
