//! CWE-434: Double extension file passes single-extension check.

// vuln-code-snippet start testcodeFileupload009
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let filename = req.param("filename");
    let content = req.body_str();
    let ext = filename.rsplit('.').next().unwrap_or("");
    if ext == "txt" || ext == "csv" || ext == "log" { // vuln-code-snippet target-line testcodeFileupload009
        let path = format!("uploads/{}", filename);
        let _ = std::fs::write(&path, content.as_bytes());
        super::shared::BenchmarkResponse::ok(&format!("Saved: {}", path))
    } else {
        super::shared::BenchmarkResponse::bad_request("Unsupported file type")
    }
}
// vuln-code-snippet end testcodeFileupload009
