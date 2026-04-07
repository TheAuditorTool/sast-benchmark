//! CWE-90: Email format validation (alphanumeric plus @ and .) rejects LDAP special chars before mail filter.

fn ldap_search(base: &str, filter: &str) -> String {
    // In production: ldap3::LdapConn::new("ldap://...").search(base, Scope::Subtree, filter, vec!["cn", "mail"])
    format!("LDAP search in {} with filter {}", base, filter)
}

// vuln-code-snippet start testcodeLdapi028
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let email = req.param("email");
    if !email.chars().all(|c| c.is_alphanumeric() || c == '@' || c == '.' || c == '-' || c == '_') {
        return super::shared::BenchmarkResponse::bad_request("invalid email");
    }
    let filter = format!("(mail={})", email);
    let result = ldap_search("dc=example,dc=com", &filter); // vuln-code-snippet target-line testcodeLdapi028
    super::shared::BenchmarkResponse::ok(&result)
}
// vuln-code-snippet end testcodeLdapi028
