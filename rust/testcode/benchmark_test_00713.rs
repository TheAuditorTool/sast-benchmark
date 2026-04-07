pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let action = req.param("action");
    let secret = fetch_from_vault("app/db-password");
    let result = format!("Action {} with vault secret", action);
    super::shared::BenchmarkResponse::ok(&result)
}

fn fetch_from_vault(path: &str) -> String {
    format!("vault_secret_{}", path)
}
