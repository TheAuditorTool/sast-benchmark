pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let user = req.param("user");
    let mut fields = vec!["event=login".to_string()];
    fields.push(format!("user={}", user));
    fields.push("result=ok".to_string());
    let log_line = fields.join(" ");
    log_info(&log_line);
    super::shared::BenchmarkResponse::ok("OK")
}

fn log_info(msg: &str) {
    eprintln!("[INFO] {}", msg);
}
