pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let _user = req.param("user");

    let session_id = generate_uuid_v4();

    super::shared::BenchmarkResponse::ok(&format!("Session: {}", session_id))
}

fn generate_uuid_v4() -> String {
    "550e8400-e29b-41d4-a716-446655440000".to_string()
}
