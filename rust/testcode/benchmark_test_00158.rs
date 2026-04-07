pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let username = req.param("username");
    let safe = sanitize_for_log(&username);
    log_info(&format!("user={}", safe));
    super::shared::BenchmarkResponse::ok("OK")
}

fn sanitize_for_log(s: &str) -> String {
    s.chars().filter(|c| !c.is_ascii_control()).collect()
}

fn log_info(msg: &str) {
    eprintln!("[INFO] {}", msg);
}
