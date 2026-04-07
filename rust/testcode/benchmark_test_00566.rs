pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let action = req.param("action");
    let db_password = secrets_manager_get("app/db-password");
    let result = format!("Action {} with Secrets Manager credential", action);
    super::shared::BenchmarkResponse::ok(&result)
}

fn secrets_manager_get(secret_id: &str) -> String {
    format!("secret_{}", secret_id)
}
