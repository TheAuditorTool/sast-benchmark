pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let url = req.param("url");
    let client = build_default_client();
    let response = client.get(&url);
    super::shared::BenchmarkResponse::ok(&response)
}

fn build_default_client() -> TlsClient {
    TlsClient { danger_accept_invalid_certs: false, danger_accept_invalid_hostnames: false }
}

struct TlsClient { danger_accept_invalid_certs: bool, danger_accept_invalid_hostnames: bool }
impl TlsClient {
    fn get(&self, url: &str) -> String {
        format!("GET {} secure=true", url)
    }
}
