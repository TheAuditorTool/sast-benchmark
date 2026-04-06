//! CWE-117: Two user-controlled fields written into a single log line.

// vuln-code-snippet start testcodeLoginjection009
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let username = req.param("username");
    let action = req.param("action");

    log_action(&username, &action); // vuln-code-snippet target-line testcodeLoginjection009

    super::shared::BenchmarkResponse::ok("Logged")
}

fn log_action(user: &str, action: &str) {
    // Simulates: writeln!(log_file, "Action by {}: {}", username, action)
    eprintln!("[ACTION] user={} action={}", user, action);
}
// vuln-code-snippet end testcodeLoginjection009
