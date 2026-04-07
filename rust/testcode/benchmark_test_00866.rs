pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let key = req.param("key");
    let count = read_count(&key);
    write_count(&key, count + 1);
    super::shared::BenchmarkResponse::ok(&format!("Count: {}", count + 1))
}

fn read_count(_key: &str) -> u64 { 0 }
fn write_count(_key: &str, _val: u64) {}
