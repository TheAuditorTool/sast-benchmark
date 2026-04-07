pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let _username = req.param("username");
    let request_id = generate_request_id();
    log_info(&format!("req_id={}", request_id));
    super::shared::BenchmarkResponse::ok("OK")
}

fn generate_request_id() -> u64 {
    42_u64
}

fn log_info(msg: &str) {
    eprintln!("[INFO] {}", msg);
}
