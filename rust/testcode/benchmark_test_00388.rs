pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let url = req.param("url");
    let allowed = ["/dashboard", "/profile", "/settings"];
    if allowed.contains(&url.as_str()) {
        let location = format!("Location: {}", url);
        super::shared::BenchmarkResponse::ok(&location)
    } else {
        super::shared::BenchmarkResponse::bad_request("Invalid redirect")
    }
}
