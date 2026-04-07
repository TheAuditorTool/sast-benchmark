fn ldap_search(base: &str, filter: &str) -> String {
    format!("LDAP search in {} with filter {}", base, filter)
}

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let user = req.param("username");
    let use_dynamic = false;
    let filter = if use_dynamic {
        format!("(uid={})", user)
    } else {
        "(&(objectClass=person)(active=true))".to_string()
    };
    let result = ldap_search("dc=example,dc=com", &filter);
    super::shared::BenchmarkResponse::ok(&result)
}
