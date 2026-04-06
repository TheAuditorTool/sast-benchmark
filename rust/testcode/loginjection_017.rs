//! CWE-117: Secret value wrapped in type that displays as [REDACTED].

// vuln-code-snippet start testcodeLoginjection017
struct RedactedSecret(String);

impl std::fmt::Display for RedactedSecret {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "[REDACTED]") // vuln-code-snippet target-line testcodeLoginjection017
    }
}

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let token = req.header("authorization");
    let secret = RedactedSecret(token);

    eprintln!("[INFO] auth={}", secret);
    super::shared::BenchmarkResponse::ok("Logged")
}
// vuln-code-snippet end testcodeLoginjection017
