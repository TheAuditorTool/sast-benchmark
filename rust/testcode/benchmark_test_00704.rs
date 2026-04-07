fn ldap_search(base: &str, filter: &str) -> String {
    format!("LDAP search in {} with filter {}", base, filter)
}

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let dept = req.param("dept");
    let valid_depts = ["Engineering", "Marketing", "Finance", "HR", "Operations"];
    if !valid_depts.contains(&dept.as_str()) {
        return super::shared::BenchmarkResponse::bad_request("unknown department");
    }
    let filter = format!("(ou={})", dept);
    let result = ldap_search("dc=example,dc=com", &filter);
    super::shared::BenchmarkResponse::ok(&result)
}
