//! CWE-287: user_id from request body trusted without session verification — grants super-user access.

fn super_user_access(uid: &str) -> super::shared::BenchmarkResponse {
    super::shared::BenchmarkResponse::ok(&format!("super user access for uid={}", uid))
}

fn regular_access(uid: &str) -> super::shared::BenchmarkResponse {
    super::shared::BenchmarkResponse::ok(&format!("access for uid={}", uid))
}

// vuln-code-snippet start testcodeAuthnfailure018
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    // user_id comes from the request body, not from a verified server-side session.
    let uid = req.param("user_id");

    if uid.is_empty() {
        return super::shared::BenchmarkResponse::bad_request("missing user_id");
    }

    // Comparing attacker-supplied uid to a privileged constant.
    if uid == "1" {
        return super_user_access(&uid); // vuln-code-snippet target-line testcodeAuthnfailure018
    }

    regular_access(&uid)
}
// vuln-code-snippet end testcodeAuthnfailure018
