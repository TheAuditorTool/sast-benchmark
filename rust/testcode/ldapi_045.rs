//! CWE-90: Filter builder ignores its input parameter and always returns a hardcoded safe filter.

fn ldap_search(base: &str, filter: &str) -> String {
    // In production: ldap3::LdapConn::new("ldap://...").search(base, Scope::Subtree, filter, vec!["cn", "mail"])
    format!("LDAP search in {} with filter {}", base, filter)
}

fn build_safe_filter(_user_input: &str) -> &'static str {
    "(&(objectClass=person)(active=true))"
}

// vuln-code-snippet start testcodeLdapi045
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let user = req.param("username");
    let filter = build_safe_filter(&user);
    let result = ldap_search("dc=example,dc=com", filter); // vuln-code-snippet target-line testcodeLdapi045
    super::shared::BenchmarkResponse::ok(&result)
}
// vuln-code-snippet end testcodeLdapi045
