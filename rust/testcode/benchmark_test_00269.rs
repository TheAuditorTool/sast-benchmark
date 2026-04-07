fn ldap_search(base: &str, filter: &str) -> String {
    format!("LDAP search in {} with filter {}", base, filter)
}

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let display_name = req.param("display_name");
    let filter = "(&(objectClass=person)(active=true))";
    let ldap_result = ldap_search("dc=example,dc=com", filter);
    let response = format!("Hello {}, results: {}", display_name, ldap_result);
    super::shared::BenchmarkResponse::ok(&response)
}
