pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let url = req.param("url");
    let client = build_client_danger_invalid_certs(true);
    let response = client.get(&url);
    super::shared::BenchmarkResponse::ok(&response)
}

fn build_client_danger_invalid_certs(accept_invalid: bool) -> TlsClient {
    TlsClient { accept_invalid_certs: accept_invalid, accept_invalid_hostnames: false }
}

struct TlsClient { accept_invalid_certs: bool, accept_invalid_hostnames: bool }

impl TlsClient {
    fn get(&self, url: &str) -> String {
        format!("GET {} (certs_verified={})", url, !self.accept_invalid_certs)
    }
}
