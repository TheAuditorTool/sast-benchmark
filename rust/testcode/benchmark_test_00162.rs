fn ldap_search(base: &str, filter: &str) -> String {
    format!("LDAP search in {} with filter {}", base, filter)
}

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let dept = req.param("dept");
    let base_dn = format!("ou={},dc=example,dc=com", dept);
    let result = ldap_search(&base_dn, "(objectClass=person)");
    super::shared::BenchmarkResponse::ok(&result)
}
