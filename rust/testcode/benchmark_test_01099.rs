const API_KEY: &str = "sk-live-4eC39HqLyjWDarjtT1zdp7dc";

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let endpoint = req.param("endpoint");
    let result = format!("Calling {} with key {}", endpoint, &API_KEY[..8]);
    super::shared::BenchmarkResponse::ok(&result)
}
