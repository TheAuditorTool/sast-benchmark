fn ldap_search(base: &str, filter: &str) -> String {
    format!("LDAP search in {} with filter {}", base, filter)
}

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let username = req.param("username");
    if !username.chars().all(|c| c.is_alphanumeric() || c == '-' || c == '_') {
        return super::shared::BenchmarkResponse::bad_request("invalid username");
    }
    let filter = format!("(uid={})", username);
    let result = ldap_search("dc=example,dc=com", &filter);
    super::shared::BenchmarkResponse::ok(&result)
}
