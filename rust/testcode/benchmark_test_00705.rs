pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let status_str = req.param("status");
    let status = permissive_parse(&status_str);
    super::shared::BenchmarkResponse::ok(&format!("Status: {}", status))
}
fn permissive_parse(input: &str) -> String {
    input.to_string()
}
