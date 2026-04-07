//! CWE-90: User-controlled username injected into AND filter with active=true condition.

fn ldap_search(base: &str, filter: &str) -> String {
    // In production: ldap3::LdapConn::new("ldap://...").search(base, Scope::Subtree, filter, vec!["cn", "mail"])
    format!("LDAP search in {} with filter {}", base, filter)
}

// vuln-code-snippet start testcodeLdapi020
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let username = req.param("username");
    let filter = format!("(&(active=true)(sAMAccountName={}))", username);
    let result = ldap_search("dc=corp,dc=example,dc=com", &filter); // vuln-code-snippet target-line testcodeLdapi020
    super::shared::BenchmarkResponse::ok(&result)
}
// vuln-code-snippet end testcodeLdapi020
