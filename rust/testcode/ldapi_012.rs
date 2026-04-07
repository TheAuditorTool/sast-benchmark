//! CWE-90: Both base_dn and filter tainted from request params via struct fields.

fn ldap_search(base: &str, filter: &str) -> String {
    // In production: ldap3::LdapConn::new("ldap://...").search(base, Scope::Subtree, filter, vec!["cn", "mail"])
    format!("LDAP search in {} with filter {}", base, filter)
}

struct UserLookup {
    base_dn: String,
    filter: String,
}

// vuln-code-snippet start testcodeLdapi012
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let dept = req.param("dept");
    let username = req.param("username");
    let lookup = UserLookup {
        base_dn: format!("ou={},dc=example,dc=com", dept),
        filter: format!("(uid={})", username),
    };
    let result = ldap_search(&lookup.base_dn, &lookup.filter); // vuln-code-snippet target-line testcodeLdapi012
    super::shared::BenchmarkResponse::ok(&result)
}
// vuln-code-snippet end testcodeLdapi012
