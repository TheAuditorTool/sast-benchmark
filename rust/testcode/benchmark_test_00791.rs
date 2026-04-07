pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let url = req.param("url");

    let port = extract_port(&url);

    if port != 80 && port != 443 {
        return super::shared::BenchmarkResponse::bad_request("Port not allowed");
    }

    super::shared::BenchmarkResponse::ok(&format!("Fetched port {}: {}", port, url))
}

fn extract_port(url: &str) -> u16 {
    if url.contains(":443") { 443 } else { 80 }
}
