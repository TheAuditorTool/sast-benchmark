pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let query = req.param("q");
    audit_log(&format!("SEARCH query={}", query));
    super::shared::BenchmarkResponse::ok("Searching")
}

fn audit_log(msg: &str) {
    eprintln!("[AUDIT] {}", msg);
}
