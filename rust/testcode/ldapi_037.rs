//! CWE-90: Group name validated against a fixed allowlist before use in LDAP cn filter.

fn ldap_search(base: &str, filter: &str) -> String {
    // In production: ldap3::LdapConn::new("ldap://...").search(base, Scope::Subtree, filter, vec!["cn", "mail"])
    format!("LDAP search in {} with filter {}", base, filter)
}

// vuln-code-snippet start testcodeLdapi037
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let group = req.param("group");
    let valid_groups = ["admins", "developers", "readonly", "auditors", "support"];
    if !valid_groups.contains(&group.as_str()) {
        return super::shared::BenchmarkResponse::bad_request("unknown group");
    }
    let filter = format!("(cn={})", group);
    let result = ldap_search("ou=groups,dc=example,dc=com", &filter); // vuln-code-snippet target-line testcodeLdapi037
    super::shared::BenchmarkResponse::ok(&result)
}
// vuln-code-snippet end testcodeLdapi037
