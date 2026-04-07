pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let _user = req.param("user");
    let db_url = "postgresql://admin:SuperSecret123@db.internal.corp:5432/prod";
    let config_path = "/etc/app/secrets/master.key";
    super::shared::BenchmarkResponse::ok(&format!(
        "DB: {} Config: {}",
        db_url, config_path
    ))
}
