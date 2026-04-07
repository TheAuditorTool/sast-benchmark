//! CWE-532: User ID hashed before logging; sensitive identifier never appears in log.

// vuln-code-snippet start testcodeInfodisclosure041
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let user_id = req.param("user_id");
    let id_hash = hash_for_log(&user_id);
    log_info(&format!("user_hash={}", id_hash)); // vuln-code-snippet target-line testcodeInfodisclosure041
    super::shared::BenchmarkResponse::ok("OK")
}

fn hash_for_log(val: &str) -> u64 {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    let mut h = DefaultHasher::new();
    val.hash(&mut h);
    h.finish()
}

fn log_info(msg: &str) { eprintln!("[INFO] {}", msg); }
// vuln-code-snippet end testcodeInfodisclosure041
