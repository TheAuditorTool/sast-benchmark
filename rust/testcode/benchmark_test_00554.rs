fn ldap_search(base: &str, filter: &str) -> String {
    format!("LDAP search in {} with filter {}", base, filter)
}

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let user = req.param("user");
    let filter = format!("(sAMAccountName={})", user);
    let result = ldap_search("dc=corp,dc=example,dc=com", &filter);
    super::shared::BenchmarkResponse::ok(&result)
}
