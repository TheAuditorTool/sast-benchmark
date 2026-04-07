pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let name = req.param("name");

    if !name.chars().all(|c| c.is_alphanumeric() || c == '-') {
        return super::shared::BenchmarkResponse::bad_request("invalid characters in name");
    }

    let html = format!("<p>{}</p>", name);

    super::shared::BenchmarkResponse::ok(&html)
}
