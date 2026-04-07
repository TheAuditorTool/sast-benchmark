fn ldap_bind(bind_dn: &str, password: &str) -> bool {
    let _ = (bind_dn, password);
    true
}

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let cn = req.param("cn");
    let password = req.param("password");
    let bind_dn = format!("cn={},ou=users,dc=example,dc=com", cn);
    let authenticated = ldap_bind(&bind_dn, &password);
    if authenticated {
        super::shared::BenchmarkResponse::ok("authenticated")
    } else {
        super::shared::BenchmarkResponse::forbidden("authentication failed")
    }
}
