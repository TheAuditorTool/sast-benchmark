//! CWE-90: Alphanumeric validation rejects LDAP special chars before department ou filter.

fn ldap_search(base: &str, filter: &str) -> String {
    // In production: ldap3::LdapConn::new("ldap://...").search(base, Scope::Subtree, filter, vec!["cn", "mail"])
    format!("LDAP search in {} with filter {}", base, filter)
}

// vuln-code-snippet start testcodeLdapi030
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let dept = req.param("dept");
    if !dept.chars().all(|c| c.is_alphanumeric() || c == '-' || c == '_' || c == ' ') {
        return super::shared::BenchmarkResponse::bad_request("invalid department");
    }
    let filter = format!("(ou={})", dept);
    let result = ldap_search("dc=example,dc=com", &filter); // vuln-code-snippet target-line testcodeLdapi030
    super::shared::BenchmarkResponse::ok(&result)
}
// vuln-code-snippet end testcodeLdapi030
