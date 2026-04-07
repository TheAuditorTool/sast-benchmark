pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let name = req.param("name");

    let body = format!("Hello, {}! This is a plain text response.", name);

    let mut resp = super::shared::BenchmarkResponse::ok(&body);
    resp.body = format!("Content-Type: text/plain\n\n{}", body);
    resp
}
