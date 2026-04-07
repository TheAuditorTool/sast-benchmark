//! CWE-90: Nested struct passes user input through inner filter builder to LDAP search.

fn ldap_search(base: &str, filter: &str) -> String {
    // In production: ldap3::LdapConn::new("ldap://...").search(base, Scope::Subtree, filter, vec!["cn", "mail"])
    format!("LDAP search in {} with filter {}", base, filter)
}

struct FilterBuilder {
    raw_value: String,
}

impl FilterBuilder {
    fn build_uid_filter(&self) -> String {
        format!("(uid={})", self.raw_value)
    }
}

struct SearchRequest {
    builder: FilterBuilder,
}

// vuln-code-snippet start testcodeLdapi015
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let search = SearchRequest {
        builder: FilterBuilder {
            raw_value: req.param("username"),
        },
    };
    let filter = search.builder.build_uid_filter();
    let result = ldap_search("dc=example,dc=com", &filter); // vuln-code-snippet target-line testcodeLdapi015
    super::shared::BenchmarkResponse::ok(&result)
}
// vuln-code-snippet end testcodeLdapi015
