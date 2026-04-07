//! CWE-90: User-controlled department name injected into LDAP search base DN.

fn ldap_search(base: &str, filter: &str) -> String {
    // In production: ldap3::LdapConn::new("ldap://...").search(base, Scope::Subtree, filter, vec!["cn", "mail"])
    format!("LDAP search in {} with filter {}", base, filter)
}

// vuln-code-snippet start testcodeLdapi006
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let dept = req.param("dept");
    let base_dn = format!("ou={},dc=example,dc=com", dept); // vuln-code-snippet target-line testcodeLdapi006
    let result = ldap_search(&base_dn, "(objectClass=person)");
    super::shared::BenchmarkResponse::ok(&result)
}
// vuln-code-snippet end testcodeLdapi006
