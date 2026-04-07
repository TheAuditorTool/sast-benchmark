//! CWE-22: Tainted entry inserted then immediately removed from Vec; safe element used for read.

// vuln-code-snippet start testcodePathtraver032
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let user_path = req.param("f");
    let mut paths = vec!["/safe/default.txt".to_string()];
    paths.push(user_path);
    paths.push("/safe/fallback.txt".to_string());
    paths.remove(1);

    let path = &paths[0];
    match std::fs::read_to_string(path) { // vuln-code-snippet target-line testcodePathtraver032
        Ok(content) => super::shared::BenchmarkResponse::ok(&content),
        Err(e) => super::shared::BenchmarkResponse::error(&e.to_string()),
    }
}
// vuln-code-snippet end testcodePathtraver032
