//! CWE-90: Tainted filter flows through struct field into LDAP search call.

fn ldap_search(base: &str, filter: &str) -> String {
    // In production: ldap3::LdapConn::new("ldap://...").search(base, Scope::Subtree, filter, vec!["cn", "mail"])
    format!("LDAP search in {} with filter {}", base, filter)
}

struct LdapQuery {
    filter: String,
}

// vuln-code-snippet start testcodeLdapi011
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let user_input = req.param("username");
    let q = LdapQuery {
        filter: format!("(cn={})", user_input),
    };
    let result = ldap_search("dc=example,dc=com", &q.filter); // vuln-code-snippet target-line testcodeLdapi011
    super::shared::BenchmarkResponse::ok(&result)
}
// vuln-code-snippet end testcodeLdapi011
