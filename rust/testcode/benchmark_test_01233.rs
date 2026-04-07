pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let name = req.param("name");
    let html = format!("<h1>Hello, {}!</h1>", name);
    super::shared::BenchmarkResponse::ok(&html)
}
