//! CWE-918: Redirect policy set to none. No redirect following.

// vuln-code-snippet start testcodeSsrf004
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let url = req.param("url");

    let redirect_policy = "none"; // vuln-code-snippet target-line testcodeSsrf004
    if redirect_policy != "none" {
        return super::shared::BenchmarkResponse::bad_request("Redirects must be disabled for external requests");
    }

    let _resp = fetch_no_redirects(&url);

    super::shared::BenchmarkResponse::ok(&format!("Fetched (no redirects): {}", url))
}

fn fetch_no_redirects(url: &str) -> String {
    // In production: reqwest::Client::builder().redirect(Policy::none()).build().get(url).send()
    format!("Response from {} (redirects disabled)", url)
}
// vuln-code-snippet end testcodeSsrf004
