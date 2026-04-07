use std::collections::HashMap;

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let url = req.param("url");
    let mut clients: HashMap<&str, TlsClient> = HashMap::new();
    clients.insert("insecure", TlsClient { verify: false });
    clients.insert("secure", TlsClient { verify: true });
    let client = clients.get("secure").unwrap();
    let response = client.get(&url);
    super::shared::BenchmarkResponse::ok(&response)
}

struct TlsClient { verify: bool }
impl TlsClient {
    fn get(&self, url: &str) -> String { format!("GET {} verify={}", url, self.verify) }
}
