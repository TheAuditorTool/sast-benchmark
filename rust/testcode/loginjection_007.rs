//! CWE-117: JSON log line constructed with format! including unescaped user string.

// vuln-code-snippet start testcodeLoginjection007
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let username = req.param("username");

    let log_line = format!(r#"{{"event":"login","user":"{}"}}"#, username); // vuln-code-snippet target-line testcodeLoginjection007
    eprintln!("{}", log_line);

    super::shared::BenchmarkResponse::ok("Logged")
}
// vuln-code-snippet end testcodeLoginjection007
