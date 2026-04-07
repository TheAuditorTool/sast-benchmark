//! CWE-90: Alphanumeric validation rejects LDAP special chars before sAMAccountName filter.

fn ldap_search(base: &str, filter: &str) -> String {
    // In production: ldap3::LdapConn::new("ldap://...").search(base, Scope::Subtree, filter, vec!["cn", "mail"])
    format!("LDAP search in {} with filter {}", base, filter)
}

// vuln-code-snippet start testcodeLdapi029
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let user = req.param("user");
    if !user.chars().all(|c| c.is_alphanumeric() || c == '-' || c == '_') {
        return super::shared::BenchmarkResponse::bad_request("invalid account name");
    }
    let filter = format!("(sAMAccountName={})", user);
    let result = ldap_search("dc=corp,dc=example,dc=com", &filter); // vuln-code-snippet target-line testcodeLdapi029
    super::shared::BenchmarkResponse::ok(&result)
}
// vuln-code-snippet end testcodeLdapi029
