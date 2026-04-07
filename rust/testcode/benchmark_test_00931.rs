fn ldap_search(base: &str, filter: &str) -> String {
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

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let search = SearchRequest {
        builder: FilterBuilder {
            raw_value: req.param("username"),
        },
    };
    let filter = search.builder.build_uid_filter();
    let result = ldap_search("dc=example,dc=com", &filter);
    super::shared::BenchmarkResponse::ok(&result)
}
