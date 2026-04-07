use std::collections::HashMap;

fn ldap_search(base: &str, filter: &str) -> String {
    format!("LDAP search in {} with filter {}", base, filter)
}

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let user = req.param("username");
    let mut m: HashMap<&str, String> = HashMap::new();
    m.insert("user_filter", format!("(uid={})", user));
    m.insert("safe_filter", "(objectClass=person)".to_string());
    let f = m.get("safe_filter").unwrap();
    let result = ldap_search("dc=example,dc=com", f);
    super::shared::BenchmarkResponse::ok(&result)
}
