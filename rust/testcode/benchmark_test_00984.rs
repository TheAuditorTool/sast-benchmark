fn ldap_search(base: &str, filter: &str) -> String {
    format!("LDAP search in {} with filter {}", base, filter)
}

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let email = req.param("email");
    let filter = format!("(mail={})", email);
    let result = ldap_search("dc=example,dc=com", &filter);
    super::shared::BenchmarkResponse::ok(&result)
}
