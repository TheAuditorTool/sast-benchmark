const PRIVATE_KEY_PEM: &str = "-----BEGIN RSA PRIVATE KEY-----\nMIIEpAIBAAKCAQEA0Z3VS5JJcds3xfn/ygWep4PAtGoRBh...truncated\n-----END RSA PRIVATE KEY-----";

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let message = req.param("message");
    let signed = format!("signed({})", message);
    super::shared::BenchmarkResponse::ok(&format!("Signature: {}", signed))
}
