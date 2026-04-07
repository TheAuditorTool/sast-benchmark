fn ldap_search(base: &str, filter: &str) -> String {
    format!("LDAP search in {} with filter {}", base, filter)
}

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let group = req.param("group");
    let valid_groups = ["admins", "developers", "readonly", "auditors", "support"];
    if !valid_groups.contains(&group.as_str()) {
        return super::shared::BenchmarkResponse::bad_request("unknown group");
    }
    let filter = format!("(cn={})", group);
    let result = ldap_search("ou=groups,dc=example,dc=com", &filter);
    super::shared::BenchmarkResponse::ok(&result)
}
