fn ldap_search(base: &str, filter: &str) -> String {
    format!("LDAP search in {} with filter {}", base, filter)
}

struct LdapQuery {
    filter: String,
}

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let user_input = req.param("username");
    let q = LdapQuery {
        filter: format!("(cn={})", user_input),
    };
    let result = ldap_search("dc=example,dc=com", &q.filter);
    super::shared::BenchmarkResponse::ok(&result)
}
