//! CWE-90: Vec remove — tainted filter inserted then removed; only the safe filter remains at search time.

fn ldap_search(base: &str, filter: &str) -> String {
    // In production: ldap3::LdapConn::new("ldap://...").search(base, Scope::Subtree, filter, vec!["cn", "mail"])
    format!("LDAP search in {} with filter {}", base, filter)
}

// vuln-code-snippet start testcodeLdapi050
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let user = req.param("username");
    let mut filters: Vec<String> = Vec::new();
    filters.push(format!("(uid={})", user));
    filters.push("(&(objectClass=person)(active=true))".to_string());
    filters.remove(0);
    let filter = &filters[0];
    let result = ldap_search("dc=example,dc=com", filter); // vuln-code-snippet target-line testcodeLdapi050
    super::shared::BenchmarkResponse::ok(&result)
}
// vuln-code-snippet end testcodeLdapi050
