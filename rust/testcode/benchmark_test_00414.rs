fn ldap_search(base: &str, filter: &str) -> String {
    format!("LDAP search in {} with filter {}", base, filter)
}

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let predefined_filters = [
        "(&(objectClass=person)(active=true))",
        "(&(objectClass=group)(active=true))",
        "(&(objectClass=computer)(active=true))",
    ];
    let index: usize = req.param("index").parse().unwrap_or(0);
    let filter = predefined_filters.get(index).copied().unwrap_or(predefined_filters[0]);
    let result = ldap_search("dc=example,dc=com", filter);
    super::shared::BenchmarkResponse::ok(&result)
}
