fn ldap_search(base: &str, filter: &str) -> String {
    format!("LDAP search in {} with filter {}", base, filter)
}

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let group = req.param("group");
    let base_dn = format!("cn={},ou=groups,dc=example,dc=com", group);
    let result = ldap_search(&base_dn, "(objectClass=groupOfNames)");
    super::shared::BenchmarkResponse::ok(&result)
}
