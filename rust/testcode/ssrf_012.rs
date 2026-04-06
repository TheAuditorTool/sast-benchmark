//! CWE-918: Low-level HTTP client sends request to a URI constructed from user input.

use std::str::FromStr;

// vuln-code-snippet start testcodeSsrf012
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let user_uri = req.param("uri");

    let uri = match http::Uri::from_str(&user_uri) {
        Ok(u) => u,
        Err(e) => return super::shared::BenchmarkResponse::bad_request(&e.to_string()),
    };

    let request = http::Request::builder()
        .method("GET")
        .uri(uri)
        .body(())
        .unwrap();

    let _resp = hyper_client_request(request); // vuln-code-snippet target-line testcodeSsrf012

    super::shared::BenchmarkResponse::ok("Request dispatched")
}

fn hyper_client_request(req: http::Request<()>) -> String {
    // Simulates: hyper::Client::new().request(req).await
    format!("Response from {}", req.uri())
}
// vuln-code-snippet end testcodeSsrf012
