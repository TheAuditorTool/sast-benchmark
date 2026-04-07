//! CWE-22: Allowlist of valid filenames; any value not in the list is rejected before file access.

// vuln-code-snippet start testcodePathtraver033
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let filename = req.param("file");
    let allowed = ["report.pdf", "readme.txt", "data.csv"];

    if !allowed.contains(&filename.as_str()) {
        return super::shared::BenchmarkResponse::forbidden("File not permitted");
    }

    let path = format!("/files/{}", filename);
    match std::fs::read_to_string(&path) { // vuln-code-snippet target-line testcodePathtraver033
        Ok(content) => super::shared::BenchmarkResponse::ok(&content),
        Err(e) => super::shared::BenchmarkResponse::error(&e.to_string()),
    }
}
// vuln-code-snippet end testcodePathtraver033
