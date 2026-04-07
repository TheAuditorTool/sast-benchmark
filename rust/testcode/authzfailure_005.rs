//! CWE-285: IDOR — file downloaded by ID without ownership check

fn download_file(path: &str) -> Vec<u8> {
    format!("file_bytes_for_{}", path).into_bytes()
}

// vuln-code-snippet start testcodeAuthzfailure005
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let file_id = req.param("file_id");
    let bytes = download_file(&file_id); // vuln-code-snippet target-line testcodeAuthzfailure005
    super::shared::BenchmarkResponse::ok(&String::from_utf8_lossy(&bytes))
}
// vuln-code-snippet end testcodeAuthzfailure005
