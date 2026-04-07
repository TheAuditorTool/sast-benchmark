const API_KEY: &str = "sk-proj-abc123xyz789def456";

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let endpoint = req.param("endpoint");
    let auth_header = format!("Authorization: Bearer {}", API_KEY);
    let result = format!("Calling {} with {}", endpoint, auth_header);
    super::shared::BenchmarkResponse::ok(&result)
}
