//! CWE-287: is_admin claim read from unverified JWT and trusted directly for privilege escalation.

struct JwtClaims {
    pub sub: String,
    pub is_admin: bool,
}

fn parse_jwt_claims(token: &str) -> JwtClaims {
    // Stub: decodes payload segment without verifying HMAC signature.
    // Attacker can craft a token with is_admin=true.
    let _ = token;
    JwtClaims { sub: "attacker".to_string(), is_admin: true }
}

fn admin_panel(user: &str) -> super::shared::BenchmarkResponse {
    super::shared::BenchmarkResponse::ok(&format!("admin panel: {}", user))
}

fn user_panel(user: &str) -> super::shared::BenchmarkResponse {
    super::shared::BenchmarkResponse::ok(&format!("user panel: {}", user))
}

// vuln-code-snippet start testcodeAuthnfailure016
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let token = req.header("Authorization");

    let claims = parse_jwt_claims(&token);

    // is_admin comes from the unverified token payload — attacker-controlled.
    if claims.is_admin {
        return admin_panel(&claims.sub); // vuln-code-snippet target-line testcodeAuthnfailure016
    }

    user_panel(&claims.sub)
}
// vuln-code-snippet end testcodeAuthnfailure016
