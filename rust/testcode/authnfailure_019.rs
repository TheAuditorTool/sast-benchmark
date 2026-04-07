//! CWE-287: "authenticated" cookie value trusted without server-side session validation.

fn access_granted(username: &str) -> super::shared::BenchmarkResponse {
    super::shared::BenchmarkResponse::ok(&format!("access granted: {}", username))
}

// vuln-code-snippet start testcodeAuthnfailure019
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let username = req.param("username");

    // Cookie value is attacker-controlled; no server-side session lookup performed.
    if req.cookie("authenticated") == "true" {
        return access_granted(&username); // vuln-code-snippet target-line testcodeAuthnfailure019
    }

    super::shared::BenchmarkResponse::forbidden("not authenticated")
}
// vuln-code-snippet end testcodeAuthnfailure019
