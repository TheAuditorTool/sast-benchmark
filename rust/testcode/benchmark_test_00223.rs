fn ldap_search(base: &str, filter: &str) -> String {
    format!("LDAP search in {} with filter {}", base, filter)
}

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let user = req.param("username");
    let escaped = user.replace('(', "\\28").replace(')', "\\29");
    let filter = format!("(uid={})", escaped);
    let result = ldap_search("dc=example,dc=com", &filter);
    super::shared::BenchmarkResponse::ok(&result)
}
