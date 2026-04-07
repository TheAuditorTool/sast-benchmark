fn ldap_search(base: &str, filter: &str) -> String {
    format!("LDAP search in {} with filter {}", base, filter)
}

struct UserQuery {
    filter: String,
}

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let user = req.param("username");
    let q = UserQuery {
        filter: format!("(&(uid={})(active=true))", user),
    };
    let result = ldap_search("dc=example,dc=com", &q.filter);
    super::shared::BenchmarkResponse::ok(&result)
}
