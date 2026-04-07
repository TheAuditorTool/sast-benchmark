//! CWE-90: Two user-controlled params injected into LDAP OR filter — dual injection point.

fn ldap_search(base: &str, filter: &str) -> String {
    // In production: ldap3::LdapConn::new("ldap://...").search(base, Scope::Subtree, filter, vec!["cn", "mail"])
    format!("LDAP search in {} with filter {}", base, filter)
}

// vuln-code-snippet start testcodeLdapi018
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let id = req.param("id");
    let email = req.param("email");
    let filter = format!("(|(uid={})(mail={}))", id, email);
    let result = ldap_search("dc=example,dc=com", &filter); // vuln-code-snippet target-line testcodeLdapi018
    super::shared::BenchmarkResponse::ok(&result)
}
// vuln-code-snippet end testcodeLdapi018
