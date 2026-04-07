//! CWE-90: Email domain validated against allowlist before LDAP mail filter lookup.

fn ldap_search(base: &str, filter: &str) -> String {
    // In production: ldap3::LdapConn::new("ldap://...").search(base, Scope::Subtree, filter, vec!["cn", "mail"])
    format!("LDAP search in {} with filter {}", base, filter)
}

// vuln-code-snippet start testcodeLdapi038
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let email = req.param("email");
    let allowed_domains = ["example.com", "corp.example.com", "partner.example.com"];
    let domain = email.split('@').nth(1).unwrap_or("");
    if !allowed_domains.contains(&domain) {
        return super::shared::BenchmarkResponse::bad_request("email domain not allowed");
    }
    let filter = format!("(mail={})", email);
    let result = ldap_search("dc=example,dc=com", &filter); // vuln-code-snippet target-line testcodeLdapi038
    super::shared::BenchmarkResponse::ok(&result)
}
// vuln-code-snippet end testcodeLdapi038
