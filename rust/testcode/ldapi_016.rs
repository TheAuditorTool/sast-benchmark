//! CWE-90: User-controlled uid injected into multi-condition AND filter with objectClass.

fn ldap_search(base: &str, filter: &str) -> String {
    // In production: ldap3::LdapConn::new("ldap://...").search(base, Scope::Subtree, filter, vec!["cn", "mail"])
    format!("LDAP search in {} with filter {}", base, filter)
}

// vuln-code-snippet start testcodeLdapi016
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let user = req.param("user");
    let filter = format!("(&(uid={})(objectClass=person))", user);
    let result = ldap_search("dc=example,dc=com", &filter); // vuln-code-snippet target-line testcodeLdapi016
    super::shared::BenchmarkResponse::ok(&result)
}
// vuln-code-snippet end testcodeLdapi016
