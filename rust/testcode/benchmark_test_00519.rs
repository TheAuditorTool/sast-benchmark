pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let resource = req.param("resource");
    let msg = format!("Resource not found: {}", resource);
    log_error(&msg);
    super::shared::BenchmarkResponse::bad_request("Not found")
}

fn log_error(msg: &str) {
    eprintln!("[ERROR] {}", msg);
}
