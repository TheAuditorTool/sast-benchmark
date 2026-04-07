//! CWE-90: User-controlled phone number concatenated into LDAP telephoneNumber filter.

fn ldap_search(base: &str, filter: &str) -> String {
    // In production: ldap3::LdapConn::new("ldap://...").search(base, Scope::Subtree, filter, vec!["cn", "mail"])
    format!("LDAP search in {} with filter {}", base, filter)
}

// vuln-code-snippet start testcodeLdapi005
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let phone = req.param("phone");
    let filter = format!("(telephoneNumber={})", phone); // vuln-code-snippet target-line testcodeLdapi005
    let result = ldap_search("dc=example,dc=com", &filter);
    super::shared::BenchmarkResponse::ok(&result)
}
// vuln-code-snippet end testcodeLdapi005
