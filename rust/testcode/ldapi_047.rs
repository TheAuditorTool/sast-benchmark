//! CWE-90: User input used only in response display string; LDAP filter is hardcoded and never tainted.

fn ldap_search(base: &str, filter: &str) -> String {
    // In production: ldap3::LdapConn::new("ldap://...").search(base, Scope::Subtree, filter, vec!["cn", "mail"])
    format!("LDAP search in {} with filter {}", base, filter)
}

// vuln-code-snippet start testcodeLdapi047
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let display_name = req.param("display_name");
    let filter = "(&(objectClass=person)(active=true))";
    let ldap_result = ldap_search("dc=example,dc=com", filter); // vuln-code-snippet target-line testcodeLdapi047
    let response = format!("Hello {}, results: {}", display_name, ldap_result);
    super::shared::BenchmarkResponse::ok(&response)
}
// vuln-code-snippet end testcodeLdapi047
