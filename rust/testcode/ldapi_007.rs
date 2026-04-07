//! CWE-90: User-controlled cn injected into LDAP bind DN, enabling DN injection.

fn ldap_bind(bind_dn: &str, password: &str) -> bool {
    // In production: ldap3::LdapConn::new("ldap://...").simple_bind(bind_dn, password)
    let _ = (bind_dn, password);
    true
}

// vuln-code-snippet start testcodeLdapi007
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let cn = req.param("cn");
    let password = req.param("password");
    let bind_dn = format!("cn={},ou=users,dc=example,dc=com", cn); // vuln-code-snippet target-line testcodeLdapi007
    let authenticated = ldap_bind(&bind_dn, &password);
    if authenticated {
        super::shared::BenchmarkResponse::ok("authenticated")
    } else {
        super::shared::BenchmarkResponse::forbidden("authentication failed")
    }
}
// vuln-code-snippet end testcodeLdapi007
