pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let url = req.param("url");
    if 10 > 5 {
        let safe = ["/home", "/login", "/register"];
        if safe.contains(&url.as_str()) {
            let location = format!("Location: {}", url);
            return super::shared::BenchmarkResponse::ok(&location);
        }
        super::shared::BenchmarkResponse::bad_request("Not allowed")
    } else {
        let location = format!("Location: {}", url);
        super::shared::BenchmarkResponse::ok(&location)
    }
}
