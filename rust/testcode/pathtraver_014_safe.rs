//! Path Traversal True Negative — CWE-22
//! Content-addressed storage: SHA-256 hash used as filename.

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

// vuln-code-snippet start testcodePathtraver014Safe
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let content = req.body_str();
    let base = "/var/data/cas";

    // SAFE: Filename derived from content hash, user has no path control
    let mut hasher = DefaultHasher::new();
    content.hash(&mut hasher); // vuln-code-snippet safe-line testcodePathtraver014Safe
    let hash = format!("{:x}", hasher.finish());

    let dest = format!("{}/{}.dat", base, hash);
    match std::fs::write(&dest, content.as_bytes()) {
        Ok(_) => super::shared::BenchmarkResponse::ok(&hash),
        Err(e) => super::shared::BenchmarkResponse::error(&e.to_string()),
    }
}
// vuln-code-snippet end testcodePathtraver014Safe
