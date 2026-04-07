pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let url = req.param("url");
    let client = if 5 + 5 == 10 {
        TlsClient { verify: true }
    } else {
        TlsClient { verify: false }
    };
    let response = client.get(&url);
    super::shared::BenchmarkResponse::ok(&response)
}

struct TlsClient { verify: bool }
impl TlsClient {
    fn get(&self, url: &str) -> String { format!("GET {} verify={}", url, self.verify) }
}
