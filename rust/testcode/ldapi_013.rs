//! CWE-90: User input tainted into multi-condition AND filter via struct initialization.

fn ldap_search(base: &str, filter: &str) -> String {
    // In production: ldap3::LdapConn::new("ldap://...").search(base, Scope::Subtree, filter, vec!["cn", "mail"])
    format!("LDAP search in {} with filter {}", base, filter)
}

struct UserQuery {
    filter: String,
}

// vuln-code-snippet start testcodeLdapi013
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let user = req.param("username");
    let q = UserQuery {
        filter: format!("(&(uid={})(active=true))", user),
    };
    let result = ldap_search("dc=example,dc=com", &q.filter); // vuln-code-snippet target-line testcodeLdapi013
    super::shared::BenchmarkResponse::ok(&result)
}
// vuln-code-snippet end testcodeLdapi013
