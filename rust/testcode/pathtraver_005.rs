//! CWE-22: file_name() extracts basename only, stripping directory components.

use std::path::Path;

// vuln-code-snippet start testcodePathtraver005
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let user_filename = req.param("filename");
    let base = "/var/www/uploads";


    let safe_name = match Path::new(&user_filename).file_name() { // vuln-code-snippet target-line testcodePathtraver005
        Some(name) => name.to_string_lossy().to_string(),
        None => return super::shared::BenchmarkResponse::bad_request("Invalid filename"),
    };

    let dest = format!("{}/{}", base, safe_name);
    match std::fs::write(&dest, req.body_str().as_bytes()) {
        Ok(_) => super::shared::BenchmarkResponse::ok("File uploaded"),
        Err(e) => super::shared::BenchmarkResponse::error(&e.to_string()),
    }
}
// vuln-code-snippet end testcodePathtraver005
