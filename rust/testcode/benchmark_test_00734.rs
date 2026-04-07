pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let url = req.param("url");
    let client = create_api_client(true);
    let response = client.get(&url);
    super::shared::BenchmarkResponse::ok(&response)
}

fn create_api_client(insecure: bool) -> TlsClient {
    configure_tls(insecure)
}

fn configure_tls(skip: bool) -> TlsClient {
    TlsClient { accept_invalid_certs: skip }
}

struct TlsClient { accept_invalid_certs: bool }
impl TlsClient {
    fn get(&self, url: &str) -> String { format!("GET {}", url) }
}
