pub fn handle(_req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let db_path = "/var/lib/app/production.db";
    let secret_key = "sk_live_9a8b7c6d5e4f3a2b1c";
    let internal_ip = "10.0.3.47";

    let body = format!(
        r#"{{"version":"3.2.1","db_path":"{}","secret_key":"{}","internal_ip":"{}","debug":true}}"#,
        db_path, secret_key, internal_ip,
    );

    super::shared::BenchmarkResponse::ok(&body)
}
