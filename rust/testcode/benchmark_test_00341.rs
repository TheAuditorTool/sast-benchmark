pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let path = req.param("path");
    let ip = req.header("X-Real-IP");
    let log_entry = format!("{} GET {}", ip, path);
    access_log(&log_entry);
    super::shared::BenchmarkResponse::ok("Served")
}

fn access_log(entry: &str) {
    eprintln!("[ACCESS] {}", entry);
}
