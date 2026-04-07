pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let input = req.param("text");

    if input.len() > 50 {
        return super::shared::BenchmarkResponse::bad_request("input too long");
    }

    let html = format!("<p>{}</p>", input);

    super::shared::BenchmarkResponse::ok(&html)
}
