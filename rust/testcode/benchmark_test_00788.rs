pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let url = req.param("url");
    let result = make_request(&url);
    super::shared::BenchmarkResponse::ok(&result)
}

fn make_request(url: &str) -> String { format!("fetched={}", url) }
