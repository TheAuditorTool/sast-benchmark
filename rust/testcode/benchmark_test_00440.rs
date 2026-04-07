fn ldap_compare(dn: &str, attribute: &str, value: &str) -> bool {
    let _ = (dn, attribute, value);
    false
}

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let uid = req.param("uid");
    let password = req.param("password");
    let dn = format!("uid={},ou=people,dc=example,dc=com", uid);
    let matches = ldap_compare(&dn, "userPassword", &password);
    if matches {
        super::shared::BenchmarkResponse::ok("password matches")
    } else {
        super::shared::BenchmarkResponse::forbidden("invalid credentials")
    }
}
