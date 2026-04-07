use std::collections::HashMap;

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let mut entries = HashMap::new();
    entries.insert("user_input", req.param("username"));
    entries.insert("safe_msg", "login_event".to_string());
    let msg = entries.get("safe_msg").unwrap();
    log_info(&format!("event={}", msg));
    super::shared::BenchmarkResponse::ok("OK")
}

fn log_info(msg: &str) {
    eprintln!("[INFO] {}", msg);
}
