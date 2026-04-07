fn ldap_search(base: &str, filter: &str) -> String {
    format!("LDAP search in {} with filter {}", base, filter)
}

struct UserLookup {
    base_dn: String,
    filter: String,
}

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let dept = req.param("dept");
    let username = req.param("username");
    let lookup = UserLookup {
        base_dn: format!("ou={},dc=example,dc=com", dept),
        filter: format!("(uid={})", username),
    };
    let result = ldap_search(&lookup.base_dn, &lookup.filter);
    super::shared::BenchmarkResponse::ok(&result)
}
