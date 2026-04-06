//! CWE-1333: Pre-validated patterns compiled into RegexSet with linear-time engine.

use regex::RegexSet;

// vuln-code-snippet start testcodeRedos021
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let text = req.param("text");
    let set = RegexSet::new(&[ // vuln-code-snippet target-line testcodeRedos021
        r"^\d{4}-\d{2}-\d{2}$",
        r"^[a-f0-9]{32}$",
        r"^[A-Z]{2}\d{6}$",
    ]).unwrap();
    let matches: Vec<_> = set.matches(&text).into_iter().collect();
    super::shared::BenchmarkResponse::ok(&format!("Matched patterns: {:?}", matches))
}
// vuln-code-snippet end testcodeRedos021
