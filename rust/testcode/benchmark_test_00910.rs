fn ldap_search(base: &str, filter: &str) -> String {
    format!("LDAP search in {} with filter {}", base, filter)
}

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let filter = match req.param("filter_type").as_str() {
        "by_name" => "(&(objectClass=person)(cn=*))",
        "by_dept" => "(&(objectClass=person)(department=*))",
        "by_email" => "(&(objectClass=person)(mail=*))",
        _ => return super::shared::BenchmarkResponse::bad_request("unknown filter type"),
    };
    let result = ldap_search("dc=example,dc=com", filter);
    super::shared::BenchmarkResponse::ok(&result)
}
