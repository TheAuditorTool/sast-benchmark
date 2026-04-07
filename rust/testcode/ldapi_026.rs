//! CWE-90: Alphanumeric-plus-dash-underscore validation rejects LDAP special characters before uid filter.

fn ldap_search(base: &str, filter: &str) -> String {
    // In production: ldap3::LdapConn::new("ldap://...").search(base, Scope::Subtree, filter, vec!["cn", "mail"])
    format!("LDAP search in {} with filter {}", base, filter)
}

// vuln-code-snippet start testcodeLdapi026
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let username = req.param("username");
    if !username.chars().all(|c| c.is_alphanumeric() || c == '-' || c == '_') {
        return super::shared::BenchmarkResponse::bad_request("invalid username");
    }
    let filter = format!("(uid={})", username);
    let result = ldap_search("dc=example,dc=com", &filter); // vuln-code-snippet target-line testcodeLdapi026
    super::shared::BenchmarkResponse::ok(&result)
}
// vuln-code-snippet end testcodeLdapi026
