//! CWE-90: Struct constructor validates input before storing; only clean values reach LDAP search.

fn ldap_search(base: &str, filter: &str) -> String {
    // In production: ldap3::LdapConn::new("ldap://...").search(base, Scope::Subtree, filter, vec!["cn", "mail"])
    format!("LDAP search in {} with filter {}", base, filter)
}

struct LdapQuery {
    filter: String,
}

impl LdapQuery {
    fn new(user: &str) -> Result<Self, String> {
        if !user.chars().all(|c| c.is_alphanumeric() || c == '-' || c == '_') {
            return Err("invalid characters in username".to_string());
        }
        Ok(Self {
            filter: format!("(uid={})", user),
        })
    }
}

// vuln-code-snippet start testcodeLdapi040
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let username = req.param("username");
    let query = match LdapQuery::new(&username) {
        Ok(q) => q,
        Err(e) => return super::shared::BenchmarkResponse::bad_request(&e),
    };
    let result = ldap_search("dc=example,dc=com", &query.filter); // vuln-code-snippet target-line testcodeLdapi040
    super::shared::BenchmarkResponse::ok(&result)
}
// vuln-code-snippet end testcodeLdapi040
