//! CWE-295: HTTP client with disabled TLS verification stored in request context struct.

// vuln-code-snippet start testcodeTlsverify029
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let url = req.param("url");
    let ctx = RequestContext { client: HttpClient { verify_tls: false } };
    let response = ctx.client.get(&url); // vuln-code-snippet target-line testcodeTlsverify029
    super::shared::BenchmarkResponse::ok(&response)
}

struct RequestContext { client: HttpClient }
struct HttpClient { verify_tls: bool }
impl HttpClient {
    fn get(&self, url: &str) -> String { format!("GET {} tls={}", url, self.verify_tls) }
}
// vuln-code-snippet end testcodeTlsverify029
