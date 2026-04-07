pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let _action = req.param("action");

    let config = actix_session_config();

    super::shared::BenchmarkResponse::ok(&format!("Session configured: {}", config))
}

fn actix_session_config() -> String {
    "secure=true,http_only=true,same_site=strict".to_string()
}
