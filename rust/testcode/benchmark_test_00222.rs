pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let body = req.body_str();
    let user = req.param("user");

    audit_log(&format!("Action by {}: {}", user, body));

    super::shared::BenchmarkResponse::ok("Audited")
}

fn audit_log(msg: &str) {
    use std::io::Write;
    if let Ok(mut f) = std::fs::OpenOptions::new().append(true).open("/var/log/audit.log") {
        let _ = writeln!(f, "{}", msg);
    }
}
