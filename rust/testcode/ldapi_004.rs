//! CWE-90: User-controlled username concatenated into Active Directory sAMAccountName filter.

fn ldap_search(base: &str, filter: &str) -> String {
    // In production: ldap3::LdapConn::new("ldap://...").search(base, Scope::Subtree, filter, vec!["cn", "mail"])
    format!("LDAP search in {} with filter {}", base, filter)
}

// vuln-code-snippet start testcodeLdapi004
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let user = req.param("user");
    let filter = format!("(sAMAccountName={})", user); // vuln-code-snippet target-line testcodeLdapi004
    let result = ldap_search("dc=corp,dc=example,dc=com", &filter);
    super::shared::BenchmarkResponse::ok(&result)
}
// vuln-code-snippet end testcodeLdapi004
