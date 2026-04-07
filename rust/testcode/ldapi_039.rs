//! CWE-90: User input maps to a pre-defined safe filter string; user never controls the filter content.

fn ldap_search(base: &str, filter: &str) -> String {
    // In production: ldap3::LdapConn::new("ldap://...").search(base, Scope::Subtree, filter, vec!["cn", "mail"])
    format!("LDAP search in {} with filter {}", base, filter)
}

// vuln-code-snippet start testcodeLdapi039
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let filter = match req.param("filter_type").as_str() {
        "by_name" => "(&(objectClass=person)(cn=*))",
        "by_dept" => "(&(objectClass=person)(department=*))",
        "by_email" => "(&(objectClass=person)(mail=*))",
        _ => return super::shared::BenchmarkResponse::bad_request("unknown filter type"),
    };
    let result = ldap_search("dc=example,dc=com", filter); // vuln-code-snippet target-line testcodeLdapi039
    super::shared::BenchmarkResponse::ok(&result)
}
// vuln-code-snippet end testcodeLdapi039
