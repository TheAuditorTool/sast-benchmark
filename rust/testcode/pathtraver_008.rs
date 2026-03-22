//! CWE-22: Hash-generated filename ignores user-supplied filename.

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

// vuln-code-snippet start testcodePathtraver008
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let base = "/var/www/uploads";


    let mut hasher = DefaultHasher::new(); // vuln-code-snippet target-line testcodePathtraver008
    std::time::SystemTime::now().hash(&mut hasher);
    let uuid = format!("{:x}", hasher.finish());
    let dest = format!("{}/{}.dat", base, uuid);

    match std::fs::write(&dest, req.body_str().as_bytes()) {
        Ok(_) => super::shared::BenchmarkResponse::ok("File uploaded"),
        Err(e) => super::shared::BenchmarkResponse::error(&e.to_string()),
    }
}
// vuln-code-snippet end testcodePathtraver008
