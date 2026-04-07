pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let query = req.param("q");

    log_info(&format!("[SEARCH] query={}", query));

    super::shared::BenchmarkResponse::ok(&format!("Searched: {}", query))
}

fn log_info(msg: &str) {
    eprintln!("{}", msg);
}
