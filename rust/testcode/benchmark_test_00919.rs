pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let name = req.param("name");

    let greeting = format!("Hello, {}!", name);
    let html = format!("<p>{}</p>", greeting);

    super::shared::BenchmarkResponse::ok(&html)
}
