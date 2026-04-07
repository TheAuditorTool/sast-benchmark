fn ldap_search(base: &str, filter: &str) -> String {
    format!("LDAP search in {} with filter {}", base, filter)
}

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let org_unit = req.param("org_unit");
    let base_dn = format!("ou={},dc=corp,dc=com", org_unit);
    let result = ldap_search(&base_dn, "(objectClass=organizationalPerson)");
    super::shared::BenchmarkResponse::ok(&result)
}
