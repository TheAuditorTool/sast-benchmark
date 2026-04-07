//! CWE-601: User supplies a numeric index mapped to a hardcoded destination list server-side.

// vuln-code-snippet start testcodeRedirect045
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let index: usize = req.param("dest").parse().unwrap_or(usize::MAX);
    let destinations = ["/home", "/profile", "/settings", "/dashboard"];
    match destinations.get(index) {
        Some(dest) => {
            let location = format!("Location: {}", dest); // vuln-code-snippet target-line testcodeRedirect045
            super::shared::BenchmarkResponse::ok(&location)
        }
        None => super::shared::BenchmarkResponse::bad_request("Invalid destination"),
    }
}
// vuln-code-snippet end testcodeRedirect045
