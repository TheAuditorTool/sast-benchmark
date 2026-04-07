pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let url = req.param("url");
    let skip_verify = std::env::var("DANGER_ACCEPT_INVALID_CERTS").is_ok();
    let client = build_tls_client(skip_verify);
    let response = client.get(&url);
    super::shared::BenchmarkResponse::ok(&response)
}

fn build_tls_client(skip: bool) -> TlsClient { TlsClient { skip_verify: skip } }

struct TlsClient { skip_verify: bool }
impl TlsClient {
    fn get(&self, url: &str) -> String { format!("GET {} skip={}", url, self.skip_verify) }
}
