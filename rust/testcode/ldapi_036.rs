//! CWE-90: Department name validated against a fixed allowlist before use in LDAP ou filter.

fn ldap_search(base: &str, filter: &str) -> String {
    // In production: ldap3::LdapConn::new("ldap://...").search(base, Scope::Subtree, filter, vec!["cn", "mail"])
    format!("LDAP search in {} with filter {}", base, filter)
}

// vuln-code-snippet start testcodeLdapi036
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let dept = req.param("dept");
    let valid_depts = ["Engineering", "Marketing", "Finance", "HR", "Operations"];
    if !valid_depts.contains(&dept.as_str()) {
        return super::shared::BenchmarkResponse::bad_request("unknown department");
    }
    let filter = format!("(ou={})", dept);
    let result = ldap_search("dc=example,dc=com", &filter); // vuln-code-snippet target-line testcodeLdapi036
    super::shared::BenchmarkResponse::ok(&result)
}
// vuln-code-snippet end testcodeLdapi036
