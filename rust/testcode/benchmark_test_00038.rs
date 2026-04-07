pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let username = req.param("username");
    let stored = format!("user:{}", username);
    let result = save_record(&stored);
    super::shared::BenchmarkResponse::ok(&result)
}

fn save_record(r: &str) -> String { format!("saved={}", r) }
