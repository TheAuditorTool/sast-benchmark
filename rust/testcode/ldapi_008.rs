//! CWE-90: User-controlled org unit injected into LDAP search base DN.

fn ldap_search(base: &str, filter: &str) -> String {
    // In production: ldap3::LdapConn::new("ldap://...").search(base, Scope::Subtree, filter, vec!["cn", "mail"])
    format!("LDAP search in {} with filter {}", base, filter)
}

// vuln-code-snippet start testcodeLdapi008
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let org_unit = req.param("org_unit");
    let base_dn = format!("ou={},dc=corp,dc=com", org_unit);
    let result = ldap_search(&base_dn, "(objectClass=organizationalPerson)"); // vuln-code-snippet target-line testcodeLdapi008
    super::shared::BenchmarkResponse::ok(&result)
}
// vuln-code-snippet end testcodeLdapi008
