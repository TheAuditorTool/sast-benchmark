pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let mut display = req.param("name");
    display = "Guest".to_string();

    let html = format!("<p>Hello, {}!</p>", display);

    super::shared::BenchmarkResponse::ok(&html)
}
