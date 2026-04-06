//! CWE-798: Database password retrieved from AWS Secrets Manager at runtime.

// vuln-code-snippet start testcodeHardcodedcreds017
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let action = req.param("action");
    // Simulates: secrets_client.get_secret_value().secret_id("app/db-password").send().await
    let db_password = secrets_manager_get("app/db-password"); // vuln-code-snippet target-line testcodeHardcodedcreds017
    let result = format!("Action {} with Secrets Manager credential", action);
    super::shared::BenchmarkResponse::ok(&result)
}

fn secrets_manager_get(secret_id: &str) -> String {
    // Simulates: aws_sdk_secretsmanager::Client::get_secret_value()
    format!("secret_{}", secret_id)
}
// vuln-code-snippet end testcodeHardcodedcreds017
