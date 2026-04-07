//! CWE-90: Tainted username stored in struct field, then used to build LDAP filter.

fn ldap_search(base: &str, filter: &str) -> String {
    // In production: ldap3::LdapConn::new("ldap://...").search(base, Scope::Subtree, filter, vec!["cn", "mail"])
    format!("LDAP search in {} with filter {}", base, filter)
}

struct AuthQuery {
    username: String,
}

// vuln-code-snippet start testcodeLdapi014
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let q = AuthQuery {
        username: req.param("username"),
    };
    let filter = format!("(uid={})", q.username);
    let result = ldap_search("dc=example,dc=com", &filter); // vuln-code-snippet target-line testcodeLdapi014
    super::shared::BenchmarkResponse::ok(&result)
}
// vuln-code-snippet end testcodeLdapi014
