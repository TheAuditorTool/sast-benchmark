fn ldap_search(base: &str, filter: &str) -> String {
    format!("LDAP search in {} with filter {}", base, filter)
}

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let dept = req.param("dept");
    if !dept.chars().all(|c| c.is_alphanumeric() || c == '-' || c == '_' || c == ' ') {
        return super::shared::BenchmarkResponse::bad_request("invalid department");
    }
    let filter = format!("(ou={})", dept);
    let result = ldap_search("dc=example,dc=com", &filter);
    super::shared::BenchmarkResponse::ok(&result)
}
