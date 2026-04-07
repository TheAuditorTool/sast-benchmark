//! CWE-614: Session cookie built from struct requiring Secure and HttpOnly during construction.

// vuln-code-snippet start testcodeSecurecookie042
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let token = req.param("token");
    let session = SecureSession::new(token);
    let cookie = session.to_cookie_header(); // vuln-code-snippet target-line testcodeSecurecookie042
    super::shared::BenchmarkResponse::ok(&format!("Set-Cookie: {}", cookie))
}

struct SecureSession { token: String }

impl SecureSession {
    fn new(token: String) -> Self { SecureSession { token } }
    fn to_cookie_header(&self) -> String {
        format!("session={}; Secure; HttpOnly; SameSite=Strict", self.token)
    }
}
// vuln-code-snippet end testcodeSecurecookie042
