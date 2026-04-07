//! CWE-90: User-controlled group name injected into LDAP group DN base.

fn ldap_search(base: &str, filter: &str) -> String {
    // In production: ldap3::LdapConn::new("ldap://...").search(base, Scope::Subtree, filter, vec!["cn", "mail"])
    format!("LDAP search in {} with filter {}", base, filter)
}

// vuln-code-snippet start testcodeLdapi009
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let group = req.param("group");
    let base_dn = format!("cn={},ou=groups,dc=example,dc=com", group);
    let result = ldap_search(&base_dn, "(objectClass=groupOfNames)"); // vuln-code-snippet target-line testcodeLdapi009
    super::shared::BenchmarkResponse::ok(&result)
}
// vuln-code-snippet end testcodeLdapi009
