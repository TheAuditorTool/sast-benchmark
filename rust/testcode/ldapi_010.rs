//! CWE-90: User-controlled uid injected into LDAP DN for direct compare operation.

fn ldap_compare(dn: &str, attribute: &str, value: &str) -> bool {
    // In production: ldap3::LdapConn::new("ldap://...").compare(dn, attribute, value)
    let _ = (dn, attribute, value);
    false
}

// vuln-code-snippet start testcodeLdapi010
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let uid = req.param("uid");
    let password = req.param("password");
    let dn = format!("uid={},ou=people,dc=example,dc=com", uid);
    let matches = ldap_compare(&dn, "userPassword", &password); // vuln-code-snippet target-line testcodeLdapi010
    if matches {
        super::shared::BenchmarkResponse::ok("password matches")
    } else {
        super::shared::BenchmarkResponse::forbidden("invalid credentials")
    }
}
// vuln-code-snippet end testcodeLdapi010
