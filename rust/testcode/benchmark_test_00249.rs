pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let url = req.param("url");
    let client = build_client_ignore_expiry();
    let response = client.get(&url);
    super::shared::BenchmarkResponse::ok(&response)
}

fn build_client_ignore_expiry() -> TlsClient {
    TlsClient { check_expiry: false, verify_cert: true }
}

struct TlsClient { check_expiry: bool, verify_cert: bool }
impl TlsClient {
    fn get(&self, url: &str) -> String {
        format!("GET {} expiry_checked={}", url, self.check_expiry)
    }
}
