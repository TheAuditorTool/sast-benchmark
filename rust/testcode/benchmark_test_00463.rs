const SSH_PRIVATE_KEY: &str = "-----BEGIN RSA PRIVATE KEY-----\nMIIEowIBAAKCAQEA...\n-----END RSA PRIVATE KEY-----";

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let host = req.param("host");
    let result = format!("Connecting to {} using embedded RSA key ({} bytes)", host, SSH_PRIVATE_KEY.len());
    super::shared::BenchmarkResponse::ok(&result)
}
