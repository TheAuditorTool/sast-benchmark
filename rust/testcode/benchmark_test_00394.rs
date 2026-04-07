pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let url = req.param("url");

    let proxy = "http://approved-proxy.internal:3128";

    let _resp = fetch_via_proxy(&url, proxy);

    super::shared::BenchmarkResponse::ok(&format!("Fetched via proxy: {}", url))
}

fn fetch_via_proxy(url: &str, proxy: &str) -> String {
    format!("Response from {} via {}", url, proxy)
}
