//! CWE-918: HTTP GET request to a URL supplied by the caller.

// vuln-code-snippet start testcodeSsrf011
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let user_url = req.param("url");

    let response = ureq::get(&user_url).call(); // vuln-code-snippet target-line testcodeSsrf011

    match response {
        Ok(resp) => {
            let body = resp.into_string().unwrap_or_default();
            super::shared::BenchmarkResponse::ok(&format!("Fetched: {}", body))
        }
        Err(e) => super::shared::BenchmarkResponse::error(&e.to_string()),
    }
}
// vuln-code-snippet end testcodeSsrf011
