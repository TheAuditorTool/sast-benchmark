//! CWE-117: Username hashed before logging; raw value never written to log output.

// vuln-code-snippet start testcodeLoginjection037
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let username = req.param("username");
    let hash = hash_for_log(&username);
    log_info(&format!("Login: user_hash={}", hash)); // vuln-code-snippet target-line testcodeLoginjection037
    super::shared::BenchmarkResponse::ok("OK")
}

fn hash_for_log(val: &str) -> u64 {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    let mut h = DefaultHasher::new();
    val.hash(&mut h);
    h.finish()
}

fn log_info(msg: &str) {
    eprintln!("[INFO] {}", msg);
}
// vuln-code-snippet end testcodeLoginjection037
