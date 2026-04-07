//! CWE-90: User input selects a pre-defined search by integer index; filter is always hardcoded.

fn ldap_search(base: &str, filter: &str) -> String {
    // In production: ldap3::LdapConn::new("ldap://...").search(base, Scope::Subtree, filter, vec!["cn", "mail"])
    format!("LDAP search in {} with filter {}", base, filter)
}

// vuln-code-snippet start testcodeLdapi046
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let predefined_filters = [
        "(&(objectClass=person)(active=true))",
        "(&(objectClass=group)(active=true))",
        "(&(objectClass=computer)(active=true))",
    ];
    let index: usize = req.param("index").parse().unwrap_or(0);
    let filter = predefined_filters.get(index).copied().unwrap_or(predefined_filters[0]);
    let result = ldap_search("dc=example,dc=com", filter); // vuln-code-snippet target-line testcodeLdapi046
    super::shared::BenchmarkResponse::ok(&result)
}
// vuln-code-snippet end testcodeLdapi046
