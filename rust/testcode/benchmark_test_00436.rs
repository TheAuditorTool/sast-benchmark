pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let recipient = req.param("recipient");
    let smtp_pwd = std::env::var("SMTP_PASSWORD").unwrap_or_default();
    let result = format!(
        "Connecting to smtp.example.com with password_len={} to send to {}",
        smtp_pwd.len(), recipient
    );
    super::shared::BenchmarkResponse::ok(&result)
}
