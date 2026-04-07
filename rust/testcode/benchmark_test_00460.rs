pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let name = req.param("name");

    let csp = "default-src 'self'; script-src 'self'; object-src 'none'";
    let html = format!("<html><body><h1>Hello, {}!</h1></body></html>", name);

    let mut resp = super::shared::BenchmarkResponse::ok(&html);
    resp.body = format!("Content-Security-Policy: {}\n\n{}", csp, html);
    resp
}
