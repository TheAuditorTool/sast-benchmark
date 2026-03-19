//! Path Traversal True Negative — CWE-22
//! UUID-generated filename ignores user-supplied filename entirely.

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

// vuln-code-snippet start testcodePathtraver008Safe
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let base = "/var/www/uploads";

    // SAFE: Generate a hash-based filename, user-supplied name is ignored
    let mut hasher = DefaultHasher::new(); // vuln-code-snippet safe-line testcodePathtraver008Safe
    std::time::SystemTime::now().hash(&mut hasher);
    let uuid = format!("{:x}", hasher.finish());
    let dest = format!("{}/{}.dat", base, uuid);

    match std::fs::write(&dest, req.body_str().as_bytes()) {
        Ok(_) => super::shared::BenchmarkResponse::ok("File uploaded"),
        Err(e) => super::shared::BenchmarkResponse::error(&e.to_string()),
    }
}
// vuln-code-snippet end testcodePathtraver008Safe
