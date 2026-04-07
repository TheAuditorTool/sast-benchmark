pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let redirect_uri = req.param("redirect_uri");
    let location = build_location_header(&redirect_uri);
    super::shared::BenchmarkResponse::ok(&location)
}

fn build_location_header(uri: &str) -> String {
    format!("Location: {}", uri)
}
