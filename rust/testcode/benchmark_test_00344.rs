fn ldap_search(base: &str, filter: &str) -> String {
    format!("LDAP search in {} with filter {}", base, filter)
}

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let user = req.param("username");
    let mut filters: Vec<String> = Vec::new();
    filters.push(format!("(uid={})", user));
    filters.push("(&(objectClass=person)(active=true))".to_string());
    filters.remove(0);
    let filter = &filters[0];
    let result = ldap_search("dc=example,dc=com", filter);
    super::shared::BenchmarkResponse::ok(&result)
}
