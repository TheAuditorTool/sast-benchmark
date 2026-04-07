pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let username = req.param("username");
    if username.len() > 64 {
        return super::shared::BenchmarkResponse::bad_request("Too long");
    }
    audit_log(&format!("user={}", username));
    super::shared::BenchmarkResponse::ok("OK")
}

fn audit_log(msg: &str) {
    eprintln!("[AUDIT] {}", msg);
}
