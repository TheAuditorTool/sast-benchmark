fn ldap_search(base: &str, filter: &str) -> String {
    format!("LDAP search in {} with filter {}", base, filter)
}

fn build_safe_filter(_user_input: &str) -> &'static str {
    "(&(objectClass=person)(active=true))"
}

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let user = req.param("username");
    let filter = build_safe_filter(&user);
    let result = ldap_search("dc=example,dc=com", filter);
    super::shared::BenchmarkResponse::ok(&result)
}
