const OAUTH_CLIENT_SECRET: &str = "oauth-client-secret-prod-2024xyz";

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let code = req.param("code");
    let result = format!(
        "POST /oauth/token client_secret={} code={}",
        OAUTH_CLIENT_SECRET, code
    );
    super::shared::BenchmarkResponse::ok(&result)
}
