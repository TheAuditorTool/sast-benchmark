pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let name = req.param("name");

    let html = format!("<html><body><h1>Hello, {}!</h1></body></html>", name);

    super::shared::BenchmarkResponse::ok(&html)
}
