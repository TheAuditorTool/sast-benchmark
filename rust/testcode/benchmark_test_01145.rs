pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let body = req.body_str();
    let parts: Vec<&str> = body.splitn(2, ':').collect();
    if parts.len() != 2 {
        return super::shared::BenchmarkResponse::bad_request("Bad format");
    }
    let sql = format!(
        "INSERT INTO audit_log (actor, action) VALUES ('{}', '{}')",
        parts[0], parts[1]
    );
    super::shared::BenchmarkResponse::ok(&format!("Logged: {}", sql))
}
