const SMTP_PASSWORD: &str = "Smtp@P4ssw0rd2024";

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let recipient = req.param("recipient");
    let result = format!(
        "Connecting to smtp.example.com with password={} to send mail to {}",
        SMTP_PASSWORD, recipient
    );
    super::shared::BenchmarkResponse::ok(&result)
}
