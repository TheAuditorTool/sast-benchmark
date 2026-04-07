//! CWE-200: File path error message uses only basename, not full server path.

// vuln-code-snippet start testcodeInfodisclosure042
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let filename = req.param("file");
    let path = format!("/var/app/data/{}", filename);
    match std::fs::read_to_string(&path) {
        Ok(c) => super::shared::BenchmarkResponse::ok(&c),
        Err(_) => {
            let basename = std::path::Path::new(&filename)
                .file_name().and_then(|n| n.to_str()).unwrap_or("file");
            super::shared::BenchmarkResponse::error(&format!("File not found: {}", basename)) // vuln-code-snippet target-line testcodeInfodisclosure042
        }
    }
}
// vuln-code-snippet end testcodeInfodisclosure042
