//! CWE-90: HashMap contains tainted filter but safe_filter key is retrieved; tainted value never reaches LDAP.

use std::collections::HashMap;

fn ldap_search(base: &str, filter: &str) -> String {
    // In production: ldap3::LdapConn::new("ldap://...").search(base, Scope::Subtree, filter, vec!["cn", "mail"])
    format!("LDAP search in {} with filter {}", base, filter)
}

// vuln-code-snippet start testcodeLdapi044
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let user = req.param("username");
    let mut m: HashMap<&str, String> = HashMap::new();
    m.insert("user_filter", format!("(uid={})", user));
    m.insert("safe_filter", "(objectClass=person)".to_string());
    let f = m.get("safe_filter").unwrap();
    let result = ldap_search("dc=example,dc=com", f); // vuln-code-snippet target-line testcodeLdapi044
    super::shared::BenchmarkResponse::ok(&result)
}
// vuln-code-snippet end testcodeLdapi044
