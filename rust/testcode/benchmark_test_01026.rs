pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let url = req.param("url");

    let redirect_policy = "none";
    if redirect_policy != "none" {
        return super::shared::BenchmarkResponse::bad_request("Redirects must be disabled for external requests");
    }

    let _resp = fetch_no_redirects(&url);

    super::shared::BenchmarkResponse::ok(&format!("Fetched (no redirects): {}", url))
}

fn fetch_no_redirects(url: &str) -> String {
    format!("Response from {} (redirects disabled)", url)
}
