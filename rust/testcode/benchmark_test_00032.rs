fn ldap_search(base: &str, filter: &str) -> String {
    format!("LDAP search in {} with filter {}", base, filter)
}

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let email = req.param("email");
    let allowed_domains = ["example.com", "corp.example.com", "partner.example.com"];
    let domain = email.split('@').nth(1).unwrap_or("");
    if !allowed_domains.contains(&domain) {
        return super::shared::BenchmarkResponse::bad_request("email domain not allowed");
    }
    let filter = format!("(mail={})", email);
    let result = ldap_search("dc=example,dc=com", &filter);
    super::shared::BenchmarkResponse::ok(&result)
}
