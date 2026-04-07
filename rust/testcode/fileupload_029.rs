//! CWE-434: Upload path constructed from user-supplied directory and filename without validation.

// vuln-code-snippet start testcodeFileupload029
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let dir = req.param("dir");
    let name = req.param("name");
    let data = req.param("data");
    let dest = build_path(&dir, &name);
    let _ = std::fs::write(&dest, data.as_bytes()); // vuln-code-snippet target-line testcodeFileupload029
    super::shared::BenchmarkResponse::ok("Uploaded")
}

fn build_path(dir: &str, name: &str) -> String {
    format!("{}/{}", dir, name)
}
// vuln-code-snippet end testcodeFileupload029
